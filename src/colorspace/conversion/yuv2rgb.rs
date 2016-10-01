use std::cmp::{min, max, Ord};
use std::ops::Deref;

use ::Surface;
use super::super::super::Channel;
use super::super::{ColorYUV, ColorRGB, Pixel};
use super::super::super::unified::{Yuv444, Rgb, Format};
use ::unsafe_impl::chunks3_mut;

fn clamp<T: Ord>(value: T, min_value: T, max_value: T) -> T {
    max(min(value, max_value), min_value)
}

#[inline(always)]
fn pixel_yuv444_to_rgb_f64(c: ColorYUV<u8>) -> ColorRGB<f64> {
    let (y, u, v) = (c.y as f64, c.u as f64, c.v as f64);
    let (up, vp) = (u - 128.0, v - 128.0);

    let r = 0.5 + y + 1.370705 * vp;
    let g = 0.5 + y - 0.698001 * vp - 0.337633 * up;
    let b = 0.5 + y + 1.732446 * up;
    
    ColorRGB { r: r, g: g, b: b }
}

#[inline(always)]
fn pixel_yuv444_to_rgb888(c: ColorYUV<u8>) -> ColorRGB<u8> {
    let rgb = pixel_yuv444_to_rgb_f64(c);
    ColorRGB {
        r: clamp(rgb.r as u32, 0, 255) as u8,
        g: clamp(rgb.g as u32, 0, 255) as u8,
        b: clamp(rgb.b as u32, 0, 255) as u8,
    }
}

pub fn yuv444_to_rgb888<S>(surf: &Surface<Yuv444, u8, S>)
    -> Surface<Rgb, u8, Box<[u8]>>
    where
        S: Deref<Target=[u8]>,
{
    let subpixel_count = surf.width() as usize * surf.height() as usize * 3;
    let mut storage: Box<[u8]> = vec![0; subpixel_count].into_boxed_slice();

    for (pin, (r, g, b)) in surf.iter_pixels().zip(chunks3_mut(&mut storage)) {
        let rgb = pixel_yuv444_to_rgb888(pin);
        *r = rgb.r;
        *g = rgb.g;
        *b = rgb.b;
    }

    Surface::new(surf.width(), surf.height(), storage)
}
