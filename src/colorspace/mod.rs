use std::marker::Reflect;
use num::traits::{Float, ToPrimitive};
use std::cmp::{min, max, Ord};
use std::ops::{Add, Sub, Mul, Div};

mod colorl;
mod colorla;
mod colorrgb;
mod colorrgba;
mod coloryuv;
pub mod conversion;

pub use self::colorl::ColorL;
pub use self::colorla::ColorLA;
pub use self::colorrgb::ColorRGB;
pub use self::colorrgba::ColorRGBA;
pub use self::coloryuv::ColorYUV;

pub trait Channel: ToPrimitive + Reflect + Clone + Copy +
        Add<Output=Self> + Sub<Output=Self> +
        Mul<Output=Self> + Div<Output=Self>
{
    // type DoubleWidth: Channel;

    #[inline(always)]
    fn from_i32(val: i32, min: i32, max: i32) -> Self;

    #[inline(always)]
    fn to_i32(&self, min: i32, max: i32) -> i32;

    fn max_depth() -> Option<u32>;
    fn min_value() -> Self;
    fn max_value() -> Self;

    fn add(a: Self, b: Self) -> Self;
    fn sub(a: Self, b: Self) -> Self;
}

impl Channel for u8 {
    // type DoubleWidth = u16;

    #[inline(always)]
    fn from_i32(val: i32, min: i32, max: i32) -> Self {
        let val_norm = (val - min) as i64;
        let range = (max - min) as i64;
        (256 * val_norm / range) as u8
    }

    #[inline(always)]
    fn to_i32(&self, min: i32, max: i32) -> i32 {
        let range = (max - min) as i64;
        let val = *self as i64;

        (val * range / 256 + min as i64) as i32
    }

    #[inline]
    fn max_depth() -> Option<u32> { Some(u8::max_value() as u32) }

    #[inline]
    fn min_value() -> u8 { u8::min_value() }

    #[inline]
    fn max_value() -> u8 { u8::max_value() }

    #[inline]
    fn add(a: u8, b: u8) -> u8 { a.saturating_add(b) }

    #[inline]
    fn sub(a: u8, b: u8) -> u8 { a.saturating_sub(b) }
}

impl Channel for u16 {
    // type DoubleWidth = u16;

    fn from_i32(val: i32, min: i32, max: i32) -> Self {
        unimplemented!();
    }

    fn to_i32(&self, min: i32, max: i32) -> i32 {
        unimplemented!();
    }

    #[inline]
    fn max_depth() -> Option<u32> { Some(u16::max_value() as u32) }

    #[inline]
    fn min_value() -> u16 { u16::min_value() }

    #[inline]
    fn max_value() -> u16 { u16::max_value() }

    #[inline]
    fn add(a: u16, b: u16) -> u16 { a.saturating_add(b) }

    #[inline]
    fn sub(a: u16, b: u16) -> u16 { a.saturating_sub(b) }
}

impl Channel for u32 {
    // type DoubleWidth = u64;

    fn from_i32(val: i32, min: i32, max: i32) -> Self {
        unimplemented!();
    }

    fn to_i32(&self, min: i32, max: i32) -> i32 {
        unimplemented!();
    }

    #[inline]
    fn max_depth() -> Option<u32> { Some(u32::max_value()) }

    #[inline]
    fn min_value() -> u32 { u32::min_value() }

    #[inline]
    fn max_value() -> u32 { u32::max_value() }

    #[inline]
    fn add(a: u32, b: u32) -> u32 { a.saturating_add(b) }

    #[inline]
    fn sub(a: u32, b: u32) -> u32 { a.saturating_sub(b) }
}

impl Channel for f64 {
    fn from_i32(val: i32, min: i32, max: i32) -> Self {
        unimplemented!();
    }

    fn to_i32(&self, min: i32, max: i32) -> i32 {
        unimplemented!();
    }

    #[inline]
    fn max_depth() -> Option<u32> { None }

    #[inline]
    fn min_value() -> f64 { 0.0 }

    #[inline]
    fn max_value() -> f64 { 1.0 }

    #[inline]
    fn add(a: f64, b: f64) -> f64 { a + b }

    #[inline]
    fn sub(a: f64, b: f64) -> f64 { a - b }
}

pub trait Colorspace: Copy + Sized {
    type Channel: Channel;

    /// Bytes per pixel
    // fn bpp() -> usize;

    fn white() -> Self;

    fn black() -> Self;

    fn luma(&self) -> Self::Channel;
}


fn clamp<T: Ord>(value: T, min_value: T, max_value: T) -> T {
    max(min(value, max_value), min_value)
}
