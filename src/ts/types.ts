export interface MeteoResponse {
  latitude: number;
  longitude: number;
  generationtime_ms: number;
  utc_offset_seconds: number;
  timezone: string;
  timezone_abbreviation: string;
  elevation: number;
  current_units: CurrentUnits;
  current: Current;
  minutely_15_units: Minutely15_Units;
  minutely_15: Minutely15;
  hourly_units: HourlyUnits;
  hourly: Hourly;
  daily_units: DailyUnits;
  daily: Daily;
}

export interface Current {
  time: number;
  interval: number;
  apparent_temperature: number;
}

export interface CurrentUnits {
  time: string;
  interval: string;
  apparent_temperature: string;
}

export interface Daily {
  time: number[];
  apparent_temperature_max: number[];
  apparent_temperature_min: number[];
}

export interface DailyUnits {
  time: string;
  apparent_temperature_max: string;
  apparent_temperature_min: string;
}

export interface Hourly {
  time: number[];
  precipitation_probability: number[];
  precipitation: number[];
  cloud_cover: number[];
}

export interface HourlyUnits {
  time: string;
  precipitation_probability: string;
  precipitation: string;
  cloud_cover: string;
}

export interface Minutely15 {
  time: number[];
  apparent_temperature: number[];
  is_day: number[];
  sunshine_duration: number[];
}

export interface Minutely15_Units {
  time: string;
  apparent_temperature: string;
  is_day: string;
  sunshine_duration: string;
}

export interface GeocodeResponse {
  address: Address;
  location: Location;
}

export interface Address {
  Match_addr: string;
  LongLabel: string;
  ShortLabel: string;
  Addr_type: string;
  Type: string;
  PlaceName: string;
  AddNum: string;
  Address: string;
  Block: string;
  Sector: string;
  Neighborhood: string;
  District: string;
  City: string;
  MetroArea: string;
  Subregion: string;
  Region: string;
  RegionAbbr: string;
  Territory: string;
  Postal: string;
  PostalExt: string;
  CntryName: string;
  CountryCode: string;
  X: number;
  Y: number;
  InputX: number;
  InputY: number;
  StrucType: string;
  StrucDet: string;
}

export interface Location {
  x: number;
  y: number;
  spatialReference: SpatialReference;
}

export interface SpatialReference {
  wkid: number;
  latestWkid: number;
}
