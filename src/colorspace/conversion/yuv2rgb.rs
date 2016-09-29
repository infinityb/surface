use std::cmp::{min, max, Ord};
use std::ops::Deref;

use ::Surface;
use super::super::super::Channel;
use super::super::{ColorYUV, ColorRGB, Colorspace};
use super::super::super::unified::{Yuv888, Rgb, ColorMode};
use ::unsafe_impl::chunks3_mut;

fn clamp<T: Ord>(value: T, min_value: T, max_value: T) -> T {
    max(min(value, max_value), min_value)
}

#[inline(always)]
fn pixel_yuv888_to_rgb888(c: ColorYUV<u8>) -> ColorRGB<u8> {
    let (y, u, v) = (c.y as f64, c.u as f64, c.v as f64);
    let (up, vp) = (u - 128.0, v - 128.0);

    let r = (0.5 + y + 1.370705 * vp) as u32;
    let g = (0.5 + y - 0.698001 * vp - 0.337633 * up) as u32;
    let b = (0.5 + y + 1.732446 * up) as u32;
    
    ColorRGB {
        r: clamp(r, 0, 255) as u8,
        g: clamp(g, 0, 255) as u8,
        b: clamp(b, 0, 255) as u8,
    }
}

pub fn yuv888_to_rgb888<S>(surf: &Surface<Yuv888, u8, S>)
    -> Surface<Rgb, u8, Box<[u8]>>
    where
        S: Deref<Target=[u8]>,
{
    let subpixel_count = surf.width() as usize * surf.height() as usize * 3;
    let mut storage: Box<[u8]> = vec![0; subpixel_count].into_boxed_slice();

    for (pin, (r, g, b)) in surf.iter_pixels().zip(chunks3_mut(&mut storage)) {
        let rgb = pixel_yuv888_to_rgb888(pin);
        *r = rgb.r;
        *g = rgb.g;
        *b = rgb.b;
    }


    Surface::new(surf.width(), surf.height(), storage)
}
