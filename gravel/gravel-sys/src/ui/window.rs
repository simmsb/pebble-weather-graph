use core::ffi::c_void;

use crate::graphics::types::GColor;
use crate::ui::clicks::{ButtonId, ClickConfigProvider, ClickHandler};
use crate::ui::layer::Layer;
use crate::util::Opaque;

unsafe extern "C" {
    pub fn window_create() -> *mut Window;
    pub fn window_destroy(window: *mut Window);
    pub fn window_set_click_config_provider(
        window: *mut Window,
        click_config_provider: ClickConfigProvider,
    );
    pub fn window_set_click_config_provider_with_context(
        window: *mut Window,
        click_config_provider: ClickConfigProvider,
        context: *mut c_void,
    );
    pub fn window_get_click_config_provider(window: *const Window) -> ClickConfigProvider;
    pub fn window_get_click_config_context(window: *mut Window) -> *mut c_void;
    pub fn window_set_window_handlers(window: *mut Window, handlers: WindowHandlers);
    pub fn window_get_root_layer(window: *const Window) -> *mut Layer;
    pub fn window_set_background_color(window: *const Window, background_color: GColor);
    pub fn window_is_loaded(window: *mut Window) -> bool;
    pub fn window_set_user_data(window: *mut Window, data: *mut c_void);
    pub fn window_get_user_data(window: *mut Window) -> *mut c_void;
    pub fn window_single_click_subscribe(button_id: ButtonId, handler: ClickHandler);
    pub fn window_single_repeating_click_subscribe(
        button_id: ButtonId,
        repeat_interval_ms: u16,
        handler: ClickHandler,
    );
    pub fn window_multi_click_subscribe(
        button_id: ButtonId,
        min_clicks: u8,
        max_clicks: u8,
        timeout: u16,
        last_click_only: bool,
        handler: ClickHandler,
    );
    pub fn window_long_click_subscribe(
        button_id: ButtonId,
        delay_ms: u16,
        down_handler: ClickHandler,
        up_handler: ClickHandler,
    );
    pub fn window_raw_click_subscribe(
        button_id: ButtonId,
        down_handler: ClickHandler,
        up_handler: ClickHandler,
        context: *mut c_void,
    );
    pub fn window_set_click_context(button_id: ButtonId, context: *mut c_void);
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct WindowHandlers {
    pub load: WindowHandler,
    pub appear: WindowHandler,
    pub disappear: WindowHandler,
    pub unload: WindowHandler,
}

impl Default for WindowHandlers {
    fn default() -> Self {
        unsafe extern "C" fn noop(_: *mut Window) {}
        Self {
            load: noop,
            appear: noop,
            disappear: noop,
            unload: noop,
        }
    }
}

#[repr(C)]
pub struct Window(Opaque);

callbacks! {
    pub fn WindowHandler(window: *mut Window);
}
