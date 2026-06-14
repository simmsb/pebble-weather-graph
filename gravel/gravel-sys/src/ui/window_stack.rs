use super::window::Window;

unsafe extern "C" {
    pub fn window_stack_push(window: *mut Window, animated: bool);
    pub fn window_stack_pop(animated: bool) -> *mut Window;
    pub fn window_stack_pop_all(animated: bool);
    pub fn window_stack_remove(window: *mut Window, animated: bool) -> bool;
    pub fn window_stack_get_top_window() -> *mut Window;
    pub fn window_stack_contains_window(window: *mut Window) -> bool;
}
