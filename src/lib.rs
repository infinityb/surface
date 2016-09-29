#![feature(reflect_marker)]

extern crate num;

//extern crate netpbm;

#[cfg(test)]
extern crate test;

pub const BOX_WIDTH_SHL: usize = 7;
pub const BOX_WIDTH: usize = 1 << 7;

pub const BOX_HEIGHT_SHL: usize = 3;
pub const BOX_HEIGHT: usize = 1 << 3;

pub use self::colorspace::{Channel, Colorspace};
pub use self::colorspace::{
	ColorL,
	ColorLA,
	ColorRGB,
	ColorRGBA,
};

pub mod colorspace;
// pub mod kernels;
pub mod unified;
// pub mod netpbm_loader;
