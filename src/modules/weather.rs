use core::{mem::MaybeUninit, u32};

use heapless::String;
use micropb::{MessageDecode, PbDecoder};
use pebble_async::{bindings, dictionary::{DictionaryIterator, TupleValue}, storage};

pub const WEATHER_NUM_DAYS: usize = 2;
pub const TL_WEATHER_SEGS_PER_HOUR: usize = 4;
pub const WEATHER_CITY_MAX_LEN: usize = 32;

const STORAGE_KEY_WEATHER_START: u32 = 1;
const STORAGE_CHUNK_SIZE: usize = 128;

pub struct Weather {
    pub data: crate::messages::Weather,
}

pub static WEATHER: static_cell::ConstStaticCell<Weather> = static_cell::ConstStaticCell::new(
    Weather {
        data: crate::messages::Weather {
            report_start: 0,
            current_temperature: 0,
            location_name: String::new(),
            segments: heapless::Vec::new(),
            hourly_segments: heapless::Vec::new(),
            daily: heapless::Vec::new(),
        },
    }
);

impl Weather {
    pub fn init(&mut self) {
        let s = unsafe {
            // TODO: this is stupid unsafe, fix it
            core::mem::transmute(self)
        };
        if Weather::try_load_from_storage(s).is_err() {
            pebble_async::info!("No saved weather or decode failed");
        }
    }

    // probably horribly broken and unsafe, but it might work
    fn try_load_from_storage(s: &mut MaybeUninit<Self>) -> Result<(), ()> {
        let byte_slice = s.as_bytes_mut();

        for (i, chunk) in byte_slice.chunks_mut(STORAGE_CHUNK_SIZE).enumerate() {
            let key = STORAGE_KEY_WEATHER_START + i as u32;
            if storage::exists(key) && storage::get_size(key) as usize == chunk.len() {
                storage::read_uninit(key, chunk).map_err(|_| ())?;
            } else {
                return Err(());
            }
        }

        Ok(())
    }

    pub fn save(&self) -> Result<(), bindings::StatusCode> {
        let byte_slice = unsafe {
            core::slice::from_raw_parts(
                (self as *const _) as *const u8,
                core::mem::size_of::<Self>(),
            )
        };

        for (i, chunk) in byte_slice.chunks(STORAGE_CHUNK_SIZE).enumerate() {
            let key = STORAGE_KEY_WEATHER_START + i as u32;
            storage::write(key, chunk)?;
        }

        Ok(())
    }

    pub fn update_from_message(&mut self, reader: &mut DictionaryIterator) -> bool {
        for tup in reader {
            if tup.key() != pebble_async::messages::WEATHER_UPDATE() {
                continue;
            }
            let TupleValue::ByteArray(buf) = tup.value() else {
                continue;
            };

            self.data = crate::messages::Weather::default();

            let mut decoder = PbDecoder::new(buf);
            decoder.ignore_repeated_cap_err = true;
            if let Err(_e) = self.data.decode(&mut decoder, buf.len()) {
                pebble_async::error!("Decode error of weather message");
            }

            if self.save().is_err() {
                pebble_async::error!("Failed to save weather");
            }
            return true;
        }

        false
    }
}
