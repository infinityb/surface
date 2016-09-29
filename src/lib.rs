#![feature(reflect_marker)]

extern crate num;

//extern crate netpbm;

#[cfg(test)]
extern crate test;

pub const BOX_WIDTH_SHL: usize = 7;
pub const BOX_WIDTH: usize = 1 << 7;

pub const BOX_HEIGHT_SHL: usize = 3;
pub const BOX_HEIGHT: usize = 1 << 3;

pub mod colorspace;
pub use self::colorspace::{Channel, Pixel};
pub use self::colorspace::{
    ColorL,
    ColorLA,
    ColorRGB,
    ColorRGBA,
};

// pub mod kernels;

// pub mod netpbm_loader;

mod unsafe_impl;
mod unified;
pub use self::unified::{
    Surface,
    Yuv888,
    Rgb,
    Rgba,
    RgbPlanar,
    RgbaPlanar,
};

mod experimental {
    use super::unified::extract_luma;
}