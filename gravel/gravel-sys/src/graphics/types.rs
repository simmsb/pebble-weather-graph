use crate::util::Opaque;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct GPoint {
    pub x: i16,
    pub y: i16,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct GSize {
    pub x: i16,
    pub y: i16,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct GRect {
    pub origin: GPoint,
    pub size: GSize,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct GColor8 {
    pub argb: u8,
}

impl GColor8 {
    pub const fn from_argb(a: u8, r: u8, g: u8, b: u8) -> Self {
        let a___ = (a & 3) << 6;
        let _r__ = (r & 3) << 4;
        let __g_ = (g & 3) << 2;
        let ___b = (b & 3) << 0;
        let argb = a___ | _r__ | __g_ | ___b;
        Self { argb }
    }
}

#[rustfmt::skip]
impl GColor8 {
    pub const fn a(&self) -> u8 { (self.argb >> 6) & 3 }
    pub const fn r(&self) -> u8 { (self.argb >> 4) & 3 }
    pub const fn g(&self) -> u8 { (self.argb >> 2) & 3 }
    pub const fn b(&self) -> u8 { (self.argb >> 0) & 3 }
}

pub type GColor = GColor8;

#[repr(C)]
pub struct GContext(Opaque);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum GAlign {
    Center,
    TopLeft,
    TopRight,
    Top,
    Left,
    Bottom,
    Right,
    BottomRight,
    BottomLeft,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum GCompOp {
    Assign,
    AssignInverted,
    Or,
    And,
    Clear,
    Set,
}
