use micropb_gen::{Config, EncodeDecode, config::IntSize};

pub const WEATHER_NUM_HOURS: usize = 48;
pub const WEATHER_NUM_DAYS: usize = 2;
pub const WEATHER_SEGS_PER_HOUR: usize = 4;
pub const WEATHER_NUM_SEGMENTS: usize = WEATHER_NUM_HOURS * WEATHER_SEGS_PER_HOUR;

fn main() {
    let mut generator = micropb_gen::Generator::new();
    generator
        .use_container_heapless()
        .configure(".", Config::new().int_size(IntSize::S8))
        .configure(
            ".Weather.segments",
            Config::new().max_len(WEATHER_NUM_SEGMENTS as u32),
        )
        .configure(
            ".Weather.hourly_segments",
            Config::new().max_len(WEATHER_NUM_HOURS as u32),
        )
        .configure(
            ".Weather.daily",
            Config::new().max_len(WEATHER_NUM_DAYS as u32),
        )
        .configure(
            ".Weather.report_start",
            Config::new().int_size(IntSize::S32),
        )
        .configure(".Weather.location_name", Config::new().max_bytes(32))
        .encode_decode(EncodeDecode::DecodeOnly)
        .calculate_max_size(true);
    generator
        .compile_protos(
            &["proto/messages.proto"],
            std::env::var("OUT_DIR").unwrap() + "/messages.rs",
        )
        .unwrap();
    println!("cargo:rerun-if-changed=messages.proto");
}
