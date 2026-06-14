unsafe extern "C" {
    pub fn sin_lookup(angle: i32) -> i32;
    pub fn cos_lookup(angle: i32) -> i32;
    pub fn atan2_lookup(y: i32, x: i32) -> i32;
}
