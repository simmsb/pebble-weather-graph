use core::{mem::MaybeUninit, u32};

use gravel::foundation::dictionary::{DictReader, DictValue};
use gravel::foundation::storage;
use gravel_sys::foundation::storage::StatusCode;
use heapless::CString;
use micropb::{MessageDecode, PbDecoder};

use crate::generated::message_keys;

pub const WEATHER_NUM_DAYS: usize = 2;
pub const TL_WEATHER_SEGS_PER_HOUR: usize = 4;
pub const WEATHER_CITY_MAX_LEN: usize = 32;

const STORAGE_KEY_WEATHER_START: u32 = 1;
const STORAGE_CHUNK_SIZE: usize = 128;

pub struct Weather {
    pub data: crate::messages::Weather,
    pub location_name: CString<{ WEATHER_CITY_MAX_LEN + 1 }>,
}

impl Default for Weather {
    fn default() -> Self {
        Self::new()
    }
}

impl Weather {
    pub fn new() -> Self {
        let mut s = MaybeUninit::<Self>::uninit();

        if Weather::try_load_from_storage(&mut s).is_err() {
            crate::info!("No saved weather or decode failed, using default");
            s.write(Self {
                data: crate::messages::Weather::default(),
                location_name: CString::new(),
            });
        }

        unsafe { s.assume_init() }
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

    pub fn save(&self) -> Result<(), StatusCode> {
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

    pub fn update_from_message(&mut self, reader: &mut DictReader<'_>) -> bool {
        for (key, value) in reader.clone() {
            if key != message_keys::MESSAGE_KEY_WEATHER_UPDATE() {
                continue;
            }
            let DictValue::ByteArray(buf) = value else {
                continue;
            };

            self.data = crate::messages::Weather::default();

            let mut decoder = PbDecoder::new(buf);
            decoder.ignore_repeated_cap_err = true;
            if let Err(e) = self.data.decode(&mut decoder, buf.len()) {
                crate::error!("Decode error: {:?}", e);
            }

            self.location_name = CString::new();
            let _ = ufmt::uwrite!(&mut self.location_name, "{}", self.data.location_name);

            if let Err(e) = self.save() {
                crate::error!("Failed to save weather: {e:?}");
            }
            return true;
        }

        false
    }
}
