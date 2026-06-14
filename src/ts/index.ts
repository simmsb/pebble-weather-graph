import { fetchJSON } from "pkts/lib/fetch";
import * as types from "./types";
import { clayConfig } from "./config";
import { sendAppMessage } from "pkts/lib/appMessage";

var WEATHER_BASE_URL = 'https://api.open-meteo.com/v1/forecast';
var GEOCODE_BASE_URL = 'https://geocode.arcgis.com/arcgis/rest/services/World/GeocodeServer/reverseGeocode';
var CACHE_KEY = 'carbon.weather.v3';
var CACHE_TTL_MS = 15 * 60 * 1000;  // 15 minutes

// @ts-ignore
const Clay = require("@rebble/clay");

var clay = new Clay(clayConfig, null, { autoHandleEvents: false });

const originalLog = console.log;

console.log = function (...args) {
  const formattedArgs = args.map(arg => {
    if (typeof arg === 'object' && arg !== null) {
      try {
        return JSON.stringify(arg, null, 2);
      } catch (err) {
        return '[Circular or Unstringifiable Object]';
      }
    }
    return arg;
  });

  originalLog.apply(console, formattedArgs);
};

function readCache(): { expiresAt: number, payload: Payload } | null {
    try {
        var raw = localStorage.getItem(CACHE_KEY);
        if (!raw) return null;
        var obj = JSON.parse(raw);
        if (!obj || !obj.payload || !obj.expiresAt) return null;
        return obj;
    } catch (e) {
        return null;
    }
}

function writeCache(payload: Payload) {
    try {
        localStorage.setItem(CACHE_KEY, JSON.stringify({
            expiresAt: Date.now() + CACHE_TTL_MS,
            payload: payload
        }));
    } catch (e) { }
}

async function sendToWatch(payload: Payload) {
    console.log("Sending payload to watch");
    const dict = {
        'WEATHER_REPORT_START': payload.fetch_time,
        'WEATHER_LOCATION_NAME': payload.city_name,
        'WEATHER_CURRENT_TEMP': payload.current_temp,
    };

    await sendAppMessage(dict);

    for (let i = 0; i < payload.daily.length; i++) {
        const daily_dict = {
            "WEATHER_DAY_UPDATE_IDX": i,
            "WEATHER_DAY_TEMP_HIGH": payload.daily[i].temp_high,
            "WEATHER_DAY_TEMP_LOW": payload.daily[i].temp_low,
            "WEATHER_DAY_NAME": payload.daily[i].day,
        }

        await sendAppMessage(daily_dict);
    }

    for (let i = 0; i < payload.hourly.length; i++) {
        const hourly_dict = {
            "WEATHER_HOUR_UPDATE_IDX": i,
            "WEATHER_HOUR_RAIN_AMOUNT": payload.hourly[i].rain_amount,
            "WEATHER_HOUR_RAIN_CHANCE": payload.hourly[i].rain_chance,
            "WEATHER_HOUR_CLOUD_COVER": payload.hourly[i].cloud_cover,
        }

        await sendAppMessage(hourly_dict);
    }

    for (let i = 0; i < payload.segment.length; i++) {
        const seg_dict = {
            "WEATHER_SEG_UPDATE_IDX": i,
            "WEATHER_SEG_TEMPERATURE": payload.segment[i].temperature,
            "WEATHER_SEG_SUN_LEVEL": payload.segment[i].sunshine_level,
            "WEATHER_SEG_IS_DAY": payload.segment[i].is_day,
        }

        await sendAppMessage(seg_dict);
    }
}

function transpose<K extends keyof any, T>(o: Record<K, T[]>): Record<K, T>[] {
    const keys = Object.keys(o);
    const length = (o as any)[keys[0]].length;

    return Array.from({ length }, (_, index) => {
        const obj = {} as any;
        keys.forEach(key => {
            obj[key] = (o as any)[key][index];
        });
        return obj;
    });
}

type Payload = {
    current_temp: number;
    daily: {
        temp_high: number;
        temp_low: number;
        day: number;
    }[];
    hourly: {
        rain_amount: number;
        rain_chance: number;
        cloud_cover: number;
    }[];
    segment: {
        temperature: number;
        sunshine_level: number;
        is_day: number;
    }[];
    fetch_time: number;
    city_name: string;
};

async function fetchAndSend(lat: number, lon: number) {
    var weatherUrl = WEATHER_BASE_URL +
        '?latitude=' + lat +
        '&longitude=' + lon +
        '&current=apparent_temperature' +
        '&minutely_15=apparent_temperature,is_day,sunshine_duration' +
        '&hourly=precipitation_probability,precipitation,cloud_cover' +
        '&daily=apparent_temperature_max,apparent_temperature_min' +
        '&forecast_days=3' +
        '&timeformat=unixtime' +
        '&timezone=auto';
    const weatherPromise = fetch(weatherUrl);

    const geocodeUrl = GEOCODE_BASE_URL + '?f=json&langCode=EN&location=' + lon + ',' + lat;
    const geocodePromise = fetch(geocodeUrl);


    const wr: types.MeteoResponse = await (await weatherPromise).json();
    var cur = wr.current;
    var hrly = wr.hourly;
    var dly = wr.daily;
    var minutely_15 = wr.minutely_15;

    const current_temp = cur.apparent_temperature;

    const daily = transpose(dly).map(x => ({
        temp_high: x.apparent_temperature_max,
        temp_low: x.apparent_temperature_min,
        day: (new Date(x.time * 1000)).getDay(),
    }));

    const hourly = transpose(hrly).map(x => ({
        rain_amount: x.precipitation,
        rain_chance: x.precipitation_probability,
        cloud_cover: 7.0 * (x.cloud_cover / 100.0),
    }));

    const segment = transpose(minutely_15).map(x => ({
        temperature: x.apparent_temperature,
        sunshine_level: 7.0 * (x.sunshine_duration / 900.0),
        is_day: x.is_day,
    }));

    const fetch_time = Math.floor(Date.now() / 1000);

    const gr: types.GeocodeResponse = await (await geocodePromise).json();
    var addr = gr && gr.address;
    const city_name = (addr && (addr.City || addr.ShortLabel)) || 'Unknown';

    const payload = {
        current_temp,
        daily,
        hourly,
        segment,
        fetch_time,
        city_name
    };

    writeCache(payload);
    try {
        await sendToWatch(payload);
    } catch (e) {
        console.log(e);
    }
}

/**
 * Check the local cache and send if still valid; otherwise acquire geolocation
 * and call fetchAndSend.
 */
async function getWeather() {
    // Check cache first
    var cache = readCache();
    if (cache && cache.expiresAt > Date.now()) {
        console.log('Carbon: using cached weather');
        // Re-evaluate unit in case locale changed; re-fetch if unit differs
        await sendToWatch(cache.payload);
        return;
    }

    const location = await new Promise<GeolocationCoordinates>((resolve, reject) => {
        navigator.geolocation.getCurrentPosition(
            function(pos) {
                console.log("Got position: " + pos.coords.latitude + " " + pos.coords.longitude);
                resolve(pos.coords);
            },
            function(err) {
                reject('Carbon: geolocation error: ' + err.message);
            },
            { timeout: 15000, maximumAge: 300000 }
        );
    });

    await fetchAndSend(location.latitude, location.longitude);
}

//
// Event listeners
//
// console.log("hello world");

Pebble.addEventListener('ready', function() {
    console.log('Carbon: PebbleKit JS ready');
    getWeather();
});

Pebble.addEventListener('showConfiguration', function() {
    Pebble.openURL(clay.generateUrl());
});

// Pebble.addEventListener('webviewclosed' as any, async function(e) {
//     if (!e.response) return;

//     // Use convert=false to get raw string-keyed settings; Clay's HTML <select>
//     // always returns string values even when the config defines number options,
//     // so we parse each integer value ourselves instead of relying on Clay's
//     // type conversion (which leaves strings as-is and breaks C int8 parsing).
//     var rawSettings = clay.getSettings(e.response, false);

//     function extractInt(setting: any) {
//         var v = (setting !== null && typeof setting === 'object' && 'value' in setting)
//             ? setting.value : setting;
//         return parseInt(v, 10);
//     }

//     function extractBool(setting: any) {
//         if (setting === null || setting === undefined) return null;
//         var v = (typeof setting === 'object' && 'value' in setting)
//             ? setting.value : setting;
//         // Guard against the string "false", which is truthy in JS.
//         if (v === 'false' || v === '0') return 0;
//         return v ? 1 : 0;
//     }

//     // await sendAppMessage(dict);
//     await getWeather();
// });

Pebble.addEventListener('appmessage', function(e) {
    if (e.payload && e.payload['WEATHER_REQUEST']) {
        console.log('Carbon: weather refresh requested');
        getWeather();
    }
});
