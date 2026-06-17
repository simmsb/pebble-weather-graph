use gravel::foundation::dictionary::{DictReader, DictValue};
use gravel::foundation::storage;
use heapless::CString;
use micropb::{MessageDecode, PbDecoder};

use crate::generated::message_keys;

// fn value_as_i32(value: DictValue<'_>) -> Option<i32> {
//     match value {
//         DictValue::I32(v) => Some(v),
//         DictValue::I8(v) => Some(v as i32),
//         DictValue::U8(v) => Some(v as i32),
//         DictValue::U32(v) => Some(v as i32),
//         _ => None,
//     }
// }

// fn copy_cstring<const N: usize>(value: DictValue<'_>, dest: &mut CString<N>) -> bool {
//     if let DictValue::CString(s) = value {
//         *dest = CString::from_bytes_truncating_at_nul(s.to_bytes_with_nul())
//             .unwrap_or_else(|_| CString::new());
//         true
//     } else {
//         false
//     }
// }

pub const WEATHER_NUM_HOURS: usize = 48;
pub const WEATHER_NUM_DAYS: usize = 2;
pub const TL_WEATHER_SEGS_PER_HOUR: usize = 4;
pub const WEATHER_NUM_SEGMENTS: usize = WEATHER_NUM_HOURS * TL_WEATHER_SEGS_PER_HOUR;
pub const WEATHER_CITY_MAX_LEN: usize = 32;

const STORAGE_KEY_WEATHER: u32 = 1;

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
        let mut s = Self {
            data: crate::messages::Weather::default(),
            location_name: CString::new(),
        };

        if storage::exists(STORAGE_KEY_WEATHER)
            && storage::get_size(STORAGE_KEY_WEATHER) as usize == core::mem::size_of::<Self>()
        {
            unsafe {
                let _ = storage::read(
                    STORAGE_KEY_WEATHER,
                    core::slice::from_raw_parts_mut(
                        &mut s as *mut _ as *mut _,
                        core::mem::size_of::<Self>(),
                    ),
                );
            }
        }

        s
    }

    pub fn save(&self) {
        let bytes = unsafe {
            core::slice::from_raw_parts(
                self as *const Self as *const u8,
                core::mem::size_of::<Self>(),
            )
        };
        let _ = storage::write(STORAGE_KEY_WEATHER, bytes);
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
                // crate::error!("Decode error: {:?}", e);
            }

            let _ = ufmt::uwrite!(&mut self.location_name, "{}", self.data.location_name);

            self.save();
            return true;
        }

        false
    }
}
