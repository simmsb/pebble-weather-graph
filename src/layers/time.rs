use core::ffi::CStr;

use futures::StreamExt as _;
use bordstein::{
    bindings::{GTextAlignment, TimeUnits},
    colour, events,
    font::Font,
    layers::{Layer, TextLayer},
    resources::Resource,
};

pub async fn time_layer(container: Layer<'_>) {
    let resource = Resource::get_handle(bordstein::bindings::RESOURCE_ID_LM_MONO_48).unwrap();
    let font = Font::load_custom_font(resource);

    let mut text_layer = container.new_child::<TextLayer>(container.bounds()).unwrap();
    text_layer.set_font(font);
    text_layer.set_background_colour(colour::GColor::CLEAR);
    text_layer.set_text_colour(colour::GColor::BLACK);
    text_layer.set_text_alignment(GTextAlignment::GTextAlignmentCenter);

    pin_init::stack_pin_init!(let tick_stream = events::tick::stream(TimeUnits::MINUTE_UNIT));

    loop {
        let mut s = [0u8; 16];
        unsafe {
            bordstein::bindings::clock_copy_time_string(s.as_mut_ptr(), 15);
        }
        let cstr = CStr::from_bytes_until_nul(&s).unwrap();

        let _tg = text_layer.set_text(cstr);

        let _ = tick_stream.next().await;
    }
}
