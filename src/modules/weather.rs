use bordstein::{
    bindings,
    dictionary::{DictionaryIterator, TupleValue},
    storage,
};
use heapless::String;
use micropb::{MessageDecode, PbDecoder, PbRead};

pub const TL_WEATHER_SEGS_PER_HOUR: usize = 4;
pub const WEATHER_CITY_MAX_LEN: usize = 32;

const STORAGE_KEY_WEATHER_START: u32 = 1;
const STORAGE_KEY_WEATHER_LEN: u32 = 1000;
const STORAGE_CHUNK_SIZE: usize = 128;

struct StorageReader {
    buf: [u8; STORAGE_CHUNK_SIZE],
    pos: usize,
    len: usize,
    next_key: u32,
    remaining: usize,
}

impl StorageReader {
    fn new(total_len: usize) -> Self {
        Self {
            buf: [0; STORAGE_CHUNK_SIZE],
            pos: 0,
            len: 0,
            next_key: STORAGE_KEY_WEATHER_START,
            remaining: total_len,
        }
    }
}

impl PbRead for StorageReader {
    type Error = ();

    fn pb_read_chunk(&mut self) -> Result<&[u8], ()> {
        if self.pos >= self.len && self.remaining > 0 {
            let want = self.remaining.min(STORAGE_CHUNK_SIZE);
            let got = storage::read(self.next_key, &mut self.buf[..want]).map_err(|_| ())?;
            if got != want {
                bordstein::error!("A weather storage chunk didn't have the expected length");
                return Err(());
            }
            self.pos = 0;
            self.len = want;
            self.next_key += 1;
            self.remaining -= want;
        }
        Ok(&self.buf[self.pos..self.len])
    }

    fn pb_advance(&mut self, bytes: usize) {
        self.pos = (self.pos + bytes).min(self.len);
    }
}

pub struct Weather {
    pub data: crate::messages::Weather,
}

pub static WEATHER: static_cell::ConstStaticCell<Weather> =
    static_cell::ConstStaticCell::new(Weather {
        data: crate::messages::Weather {
            report_start: 0,
            current_temperature: 0,
            location_name: String::new(),
            segments: heapless::Vec::new(),
            hourly_segments: heapless::Vec::new(),
            daily: heapless::Vec::new(),
        },
    });

impl Weather {
    pub fn init(&mut self) {
        if self.try_load_from_storage().is_err() {
            // A failed decode may have filled in some of the fields.
            self.data = crate::messages::Weather::default();
            bordstein::info!("No saved weather or decode failed");
        }
    }

    fn try_load_from_storage(&mut self) -> Result<(), ()> {
        let mut len = [0u8; 4];
        match storage::read(STORAGE_KEY_WEATHER_LEN, &mut len) {
            Ok(4) => {}
            _ => {
                bordstein::error!("Weather storage length not encoded as 4 bytes");
                return Err(());
            }
        }
        let len = u32::from_le_bytes(len) as usize;
        if len == 0 {
            return Err(());
        }

        let mut decoder = PbDecoder::new(StorageReader::new(len));
        decoder.ignore_repeated_cap_err = true;
        self.data.decode(&mut decoder, len).map_err(|_| {
            bordstein::error!("Failed to decode stored weather");
        })
    }

    /// Save an encoded weather message.
    fn save(encoded: &[u8]) -> Result<(), bindings::StatusCode> {
        // Invalidate first, so that an interrupted save isn't loaded.
        storage::write(STORAGE_KEY_WEATHER_LEN, &0u32.to_le_bytes())?;

        for (i, chunk) in encoded.chunks(STORAGE_CHUNK_SIZE).enumerate() {
            storage::write(STORAGE_KEY_WEATHER_START + i as u32, chunk)?;
        }

        storage::write(
            STORAGE_KEY_WEATHER_LEN,
            &(encoded.len() as u32).to_le_bytes(),
        )?;
        Ok(())
    }

    pub fn update_from_message(&mut self, reader: &mut DictionaryIterator) -> bool {
        for tup in reader {
            if tup.key() != bordstein::messages::WEATHER_UPDATE() {
                continue;
            }
            let TupleValue::ByteArray(buf) = tup.value() else {
                continue;
            };

            self.data = crate::messages::Weather::default();

            let mut decoder = PbDecoder::new(buf);
            decoder.ignore_repeated_cap_err = true;
            if let Err(_e) = self.data.decode(&mut decoder, buf.len()) {
                bordstein::error!("Decode error of weather message");
            }

            if Self::save(buf).is_err() {
                bordstein::error!("Failed to save weather");
            }
            return true;
        }

        false
    }
}
