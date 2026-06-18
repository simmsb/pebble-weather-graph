import * as types from "./types";
import { clayConfig } from "./config";
import { sendAppMessage } from "pkts/lib/appMessage";
import * as pb from "@bufbuild/protobuf";
import * as messages_pb from "./gen/messages_pb";

var WEATHER_BASE_URL = 'https://api.open-meteo.com/v1/forecast';
var GEOCODE_BASE_URL = 'https://geocode.arcgis.com/arcgis/rest/services/World/GeocodeServer/reverseGeocode';
var CACHE_KEY = 'carbon.weather.v4';
var CACHE_TTL_MS = 15 * 60 * 1000;  // 15 minutes

// @ts-ignore
const Clay = require("@rebble/clay");

const clay = new Clay(clayConfig, null, { autoHandleEvents: false });

const originalLog = console.log;

const WEATHER_NUM_HOURS = 48;
const WEATHER_NUM_DAYS = 2;
const WEATHER_SEGS_PER_HOUR = 4;
const WEATHER_NUM_SEGMENTS = WEATHER_NUM_HOURS * WEATHER_SEGS_PER_HOUR;

const FIFTEEN_MIN_SEC = 15 * 60;
const ONE_HOUR_SEC = 60 * 60;
const ONE_DAY_SEC = 24 * 60 * 60;

/** Floor to 15-minute boundary, matching the watch's utc_now rounding. */
function roundedNowSec(): number {
    const nowSec = Math.floor(Date.now() / 1000);
    return nowSec - (nowSec % FIFTEEN_MIN_SEC);
}

function findFirstAtOrAfter(times: number[], t: number): number {
    const idx = times.findIndex(time => time >= t);
    return idx === -1 ? times.length : idx;
}

/** Index of the interval containing t; intervals are [times[i], times[i+1]) or [times[i], times[i] + intervalSec). */
function findContainingIntervalIndex(times: number[], t: number, intervalSec: number): number {
    for (let i = 0; i < times.length; i++) {
        const end = i + 1 < times.length ? times[i + 1] : times[i] + intervalSec;
        if (times[i] <= t && t < end) {
            return i;
        }
    }
    return 0;
}

interface WeatherCache {
    expiresAt: number;
    weather: types.MeteoResponse;
    locationName: string;
}

console.log = function(...args) {
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
function readCache(): WeatherCache | null {
    try {
        var raw = localStorage.getItem(CACHE_KEY);
        if (!raw) return null;
        var obj = JSON.parse(raw);
        if (!obj || !obj.weather || !obj.expiresAt || !obj.locationName) return null;
        return obj;
    } catch (e) {
        return null;
    }
}

function writeCache(weather: types.MeteoResponse, locationName: string) {
    try {
        localStorage.setItem(CACHE_KEY, JSON.stringify({
            expiresAt: Date.now() + CACHE_TTL_MS,
            weather,
            locationName,
        }));
    } catch (e) { }
}

function buildPayload(weather: types.MeteoResponse, locationName: string): messages_pb.Weather | null {
    const cur = weather.current;
    const hrly = weather.hourly;
    const dly = weather.daily;
    const minutely_15 = weather.minutely_15;

    const now = roundedNowSec();
    const minutelyRows = transpose(minutely_15);
    const hourlyRows = transpose(hrly);
    const dailyRows = transpose(dly);

    const startSegIdx = findFirstAtOrAfter(minutelyRows.map(r => r.time), now);
    if (startSegIdx >= minutelyRows.length) {
        return null;
    }

    const reportStart = minutelyRows[startSegIdx].time;
    const startHourIdx = findContainingIntervalIndex(hrly.time, reportStart, ONE_HOUR_SEC);
    const startDayIdx = findContainingIntervalIndex(dly.time, reportStart, ONE_DAY_SEC);

    const daily: messages_pb.WeatherDaily[] = dailyRows.slice(startDayIdx, startDayIdx + WEATHER_NUM_DAYS).map(x => pb.create(messages_pb.WeatherDailySchema, {
        tempHigh: Math.round(x.apparent_temperature_max),
        tempLow: Math.round(x.apparent_temperature_min),
        day: (new Date(x.time * 1000)).getDay(),
    }));

    const hourlySegments: messages_pb.WeatherHourly[] = hourlyRows.slice(startHourIdx, startHourIdx + WEATHER_NUM_HOURS).map(x => pb.create(messages_pb.WeatherHourlySchema, {
        rainAmount: Math.round(x.precipitation),
        rainChance: x.precipitation_probability,
        cloudCover: Math.round(7.0 * (x.cloud_cover / 100.0)),
    }));

    const segments: messages_pb.WeatherSegment[] = minutelyRows.slice(startSegIdx, startSegIdx + WEATHER_NUM_SEGMENTS).map(x => pb.create(messages_pb.WeatherSegmentSchema, {
        temperature: Math.round(x.apparent_temperature),
        sunLevel: Math.round(7.0 * (x.sunshine_duration / 900.0)),
        isDay: !!x.is_day,
    }));

    return pb.create(messages_pb.WeatherSchema, {
        currentTemperature: Math.round(cur.apparent_temperature),
        daily,
        hourlySegments,
        segments,
        reportStart,
        locationName,
    });
}

async function sendToWatch(payload: messages_pb.Weather) {
    console.log("Sending payload to watch");

    try {
        const bytes = pb.toBinary(messages_pb.WeatherSchema, payload)
        // const asJson = pb.toJsonString(messages_pb.WeatherSchema, payload);
        // console.log(asJson);
        console.log(`Sending ${bytes.length} bytes`);

        await sendAppMessage({
            "WEATHER_UPDATE": Array.from(bytes)
        });
    } catch (e) {
        let msg;
        if (e instanceof Error) {
            msg = `${e.name}: ${e.message}`;
        } else {
            msg = JSON.stringify(e)
        }
        console.log(`SendToWatch failed: ${msg}`);
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
        '&timezone=UTC';
    const weatherPromise = fetch(weatherUrl);

    const geocodeUrl = GEOCODE_BASE_URL + '?f=json&langCode=EN&location=' + lon + ',' + lat;
    const geocodePromise = fetch(geocodeUrl);


    const wr: types.MeteoResponse = await (await weatherPromise).json();

    const gr: types.GeocodeResponse = await (await geocodePromise).json();
    var addr = gr && gr.address;
    const locationName = (addr && (addr.City || addr.ShortLabel)) || 'Unknown';

    writeCache(wr, locationName);
    await buildAndSend(wr, locationName);
}

async function buildAndSend(weather: types.MeteoResponse, locationName: string) {
    const payload = buildPayload(weather, locationName);
    if (!payload) {
        console.log("No future weather data to send");
        return;
    }
    await sendToWatch(payload);
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
        await buildAndSend(cache.weather, cache.locationName);
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
