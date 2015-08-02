use std::cmp::{min, max, Ord};

use super::super::{ColorYUV, ColorRGB, Colorspace};
use ::Surface;

// void YUVImage::yuv2rgb(uint8_t yValue, uint8_t uValue, uint8_t vValue,
//         uint8_t *r, uint8_t *g, uint8_t *b) const {
//     *r = yValue + (1.370705 * (vValue-128));
//     *g = yValue - (0.698001 * (vValue-128)) - (0.337633 * (uValue-128));
//     *b = yValue + (1.732446 * (uValue-128));
//     *r = clamp(*r, 0, 255);
//     *g = clamp(*g, 0, 255);
//     *b = clamp(*b, 0, 255);
// }

fn clamp<T: Ord>(value: T, min_value: T, max_value: T) -> T {
    max(min(value, max_value), min_value)
}

fn pixel_yuv444_to_rgb888(c: ColorYUV<u8>) -> ColorRGB<u8> {
	let (y, u, v) = (c.y as f64, c.u as f64, c.v as f64);
	let (up, vp) = (u - 128.0, v - 128.0);

    let r = (y + 1.370705 * vp) as u32;
    let g = (y - 0.698001 * vp - 0.337633 * up) as u32;
    let b = (y + 1.732446 * up) as u32;
    
    ColorRGB {
	    r: clamp(r, 0, 255) as u8,
	    g: clamp(g, 0, 255) as u8,
	    b: clamp(b, 0, 255) as u8,
	}
}

pub fn yuv444_to_rgb888(surf: &Surface<ColorYUV<u8>>) -> Surface<ColorRGB<u8>> {
	let mut out = Surface::new(surf.width, surf.height, ColorRGB::black());
	for (pin, pout) in surf.iter_pixels().zip(out.iter_pixels_mut()) {
		*pout = pixel_yuv444_to_rgb888(*pin);
	}
	out
}