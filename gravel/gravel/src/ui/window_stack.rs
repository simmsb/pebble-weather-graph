use super::window::{WindowHandle, WindowRef};

use gravel_sys::ui::window_stack as sys;

pub fn push<T>(window: &impl WindowHandle<T>, animated: bool) {
    unsafe { sys::window_stack_push(window.as_raw(), animated) }
}

pub fn pop(animated: bool) -> Option<WindowRef<()>> {
    unsafe { WindowRef::try_from_raw(sys::window_stack_pop(animated)) }
}

#[doc(alias = "clear")]
pub fn pop_all(animated: bool) {
    unsafe { sys::window_stack_pop_all(animated) }
}

pub fn remove<T>(window: &impl WindowHandle<T>, animated: bool) -> bool {
    unsafe { sys::window_stack_remove(window.as_raw(), animated) }
}

pub fn get_top() -> Option<WindowRef<()>> {
    unsafe { WindowRef::try_from_raw(sys::window_stack_get_top_window()) }
}

pub fn contains<T>(window: &impl WindowHandle<T>) -> bool {
    unsafe { sys::window_stack_contains_window(window.as_raw()) }
}
