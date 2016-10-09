use std::ops::{Deref, DerefMut};

use super::{Format};
use super::super::Channel;
use super::super::colorspace::ColorYUV as ColorYuv;

#[derive(Clone)]
pub struct Yuv444;

impl<C> Format<C> for Yuv444 where C: Channel {
    type Pixel = ColorYuv<C>;

    fn channel_data_size(width: u32, height: u32) -> usize
    {
        3 * width as usize * height as usize / 2
    }

    fn init_black(width: u32, height: u32, storage: &mut [C])
    {
        assert!(storage.len() == <Self as Format<C>>::channel_data_size(width, height));
        let pixels = width as usize * width as usize;

        let luma_min = <C as Channel>::from_i32(0, 0, 2);
        let chroma_neutral = <C as Channel>::from_i32(1, 0, 2);

        let mut iter = storage.iter_mut();
        loop {
            if let Some(y) = iter.next() {
                *y = luma_min;
            }
            if let Some(u) = iter.next() {
                *u = chroma_neutral;
            }
            if let Some(v) = iter.next() {
                *v = chroma_neutral;
            } else {
                break;
            }
        }
    }

    #[inline]
    fn get_pixel(holder: &[C], width: u32, height: u32, x: u32, y: u32) -> Self::Pixel {
        let start_subpixel = (3 * (x + width * y)) as usize;

        let mut pixel = [Channel::from_i32(0, 0, 2); 3];
        pixel.copy_from_slice(&holder[start_subpixel..][..3]);

        ColorYuv::new_yuv(pixel[0], pixel[1], pixel[2])
    }

    #[inline]
    fn put_pixel(holder: &mut [C], width: u32, height: u32, x: u32, y: u32, pixel: <Self as Format<C>>::Pixel) {
        let start_subpixel = (3 * (x + width * y)) as usize;

        holder[start_subpixel..][..3].copy_from_slice(&[pixel.y, pixel.u, pixel.v]);
    }
}


#[derive(Clone)]
pub struct Yuv444p;

impl<C> Format<C> for Yuv444p where C: Channel {
    type Pixel = ColorYuv<C>;

    fn channel_data_size(width: u32, height: u32) -> usize
    {
        3 * width as usize * height as usize
    }

    /// panicks if storage is insufficiently large.
    fn init_black(width: u32, height: u32, storage: &mut [C])
    {
        let pixels = width as usize * width as usize;
        let luma_min = <C as Channel>::from_i32(0, 0, 2);
        let chroma_neutral = <C as Channel>::from_i32(1, 0, 2);

        let mut iter = storage.iter_mut();

        let mut failed = false;
        for _ in 0..pixels {
            match iter.next() {
                Some(y) => *y = luma_min,
                None => failed = true,
            }
        }
        for _ in 0..pixels {
            match iter.next() {
                Some(u) => *u = chroma_neutral,
                None => failed = true,
            }
        }
        for _ in 0..pixels {
            match iter.next() {
                Some(v) => *v = chroma_neutral,
                None => failed = true,
            }
        }
        if failed {
            panic!("invalid storage");
        }
    }

    #[inline]
    fn get_pixel(holder: &[C], width: u32, height: u32, x: u32, y: u32) -> Self::Pixel {
        let (width, height) = (width as usize, height as usize);
        let (x, y) = (x as usize, y as usize);
        let pixels = width * height;

        ColorYuv {
            y: holder[0 * pixels + (x + y * width)],
            u: holder[1 * pixels + (x + y * width)],
            v: holder[2 * pixels + (x + y * width)],
        }
    }

    #[inline]
    fn put_pixel(holder: &mut [C], width: u32, height: u32, x: u32, y: u32, pixel: <Self as Format<C>>::Pixel) {
        let (width, height) = (width as usize, height as usize);
        let (x, y) = (x as usize, y as usize);
        let pixels = width * height;

        holder[0 * pixels + (x + y * width)] = pixel.y;
        holder[1 * pixels + (x + y * width)] = pixel.u;
        holder[2 * pixels + (x + y * width)] = pixel.v;
    }
}