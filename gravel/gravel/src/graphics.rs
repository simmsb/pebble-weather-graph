// pub mod gdraw;
// pub mod gpath;
pub mod colors;
pub mod draw;
pub mod types {
    pub use gravel_sys::graphics::types::*;
}
// pub mod text;
// pub mod fonts;
// pub mod context;

pub mod bitmap;
pub mod fonts;
pub use bitmap::Bitmap;
pub use fonts::{CustomFont, SystemFont};
pub use colors::GColor;
pub use colors::{from_hex, from_rgb, BLACK, BLUE, CLEAR, DARK_GRAY, GREEN, LIGHT_GRAY, ORANGE, PASTEL_YELLOW, RED, WHITE, YELLOW, BABY_BLUE_EYES, CELESTE, CHROME_YELLOW, ELECTRIC_ULTRAMARINE, PICTON_BLUE, SUNSET_ORANGE, VERY_LIGHT_BLUE};
pub use gravel_sys::graphics::gdraw::{GFont, GTextAlignment, GTextOverflowMode};
