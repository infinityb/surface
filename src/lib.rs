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
    ColorYUV, 
};

// pub mod kernels;

pub mod kernels;
mod unsafe_impl;
mod unified;
mod resize;

pub use self::unified::{
    surface_byte_size,
    Surface,
    Format,
    PlanarFormat,
    Yuv420p,
    Yuv422,
    Yuv422p,
    Yuv444,
    Rgb,
    Rgba,
    RgbPlanar,
    RgbaPlanar,
    Luma,
};

pub mod experimental {
    pub use super::unified::extract_luma;

    pub mod resize {
        pub use super::super::resize::resize_nearest;
    }
}