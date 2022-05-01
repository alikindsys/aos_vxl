use crate::types::types::BGRAColor;
pub const WIDTH : i32 = 512;
pub const HEIGHT : i32 = 512;
pub const DEPTH : i32 = 64;
pub const END_OF_RUN : i32 = 0;

pub const SKY_COLOR: BGRAColor = BGRAColor {
    b: 250,
    g: 206,
    r: 91,
    a: 0xFF
};

pub const DEFAULT_COLOR: BGRAColor = BGRAColor {
    b: 184,
    g: 169,
    r: 245,
    a: 0xFF
};