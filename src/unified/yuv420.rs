use std::ops::{Deref, DerefMut};

use super::{ColorMode};
use super::super::Channel;
use super::super::colorspace::ColorYUV as ColorYuv;


#[derive(Clone)]
pub struct Yuv420p;

#[inline]
pub fn get_y<C>(data: &[C], pixels: usize) -> &[C] {
    &data[..pixels]
}

#[inline]
pub fn get_u<C>(data: &[C], pixels: usize) -> &[C] {
    &data[pixels..][..pixels / 4]
}

#[inline]
pub fn get_v<C>(data: &[C], pixels: usize) -> &[C] {
    &data[pixels..][pixels / 4..][..pixels / 4]
}

#[inline]
pub fn get_y_mut<C>(data: &mut [C], pixels: usize) -> &mut [C] {
    &mut data[..pixels]
}

#[inline]
pub fn get_u_mut<C>(data: &mut [C], pixels: usize) -> &mut [C] {
    &mut data[pixels..][..pixels / 4]
}

#[inline]
pub fn get_v_mut<C>(data: &mut [C], pixels: usize) -> &mut [C] {
    &mut data[pixels..][pixels / 4..][..pixels / 4]
}

impl<C> ColorMode<C> for Yuv420p where C: Channel {
    type Pixel = ColorYuv<C>;

    fn channel_data_size(width: u32, height: u32) -> usize
    {
        3 * width as usize * height as usize / 2
    }

    fn init_black(width: u32, height: u32, storage: &mut [C])
    {
        assert!(storage.len() == <Self as ColorMode<C>>::channel_data_size(width, height));
        let pixels = width as usize * width as usize;

        let luma_min = <C as Channel>::from_i32(0, 0, 2);
        let chroma_neutral = <C as Channel>::from_i32(1, 0, 2);

        for ch in get_y_mut(storage, pixels).iter_mut() {
            *ch = luma_min;
        }
        for ch in get_u_mut(storage, pixels).iter_mut() {
            *ch = chroma_neutral;
        }
        for ch in get_v_mut(storage, pixels).iter_mut() {
            *ch = chroma_neutral;
        }
    }

    #[inline]
    fn get_pixel(holder: &[C], width: u32, height: u32, x: u32, y: u32) -> Self::Pixel {
        let pixels = width as usize * height as usize;
        let offset_y = x + width * y;
        let offset_c = (x / 2) + width * (y / 2);
        let y = get_y(holder, pixels)[offset_y as usize];
        let u = get_u(holder, pixels)[offset_c as usize];
        let v = get_v(holder, pixels)[offset_c as usize];
        ColorYuv::new_yuv(y, u, v)
    }

    #[inline]
    fn put_pixel(holder: &mut [C], width: u32, height: u32, x: u32, y: u32, pixel: <Self as ColorMode<C>>::Pixel) {
        let pixels = width as usize * height as usize;
        let offset_y = x + width * y;
        let offset_c = (x / 2) + width * (y / 2);
        get_y_mut(holder, pixels)[offset_y as usize] = pixel.y;
        get_u_mut(holder, pixels)[offset_c as usize] = pixel.u;
        get_v_mut(holder, pixels)[offset_c as usize] = pixel.v;
    }
}



#[derive(Clone)]
pub struct Yuv888;


impl<C> ColorMode<C> for Yuv888 where C: Channel {
    type Pixel = ColorYuv<C>;

    fn channel_data_size(width: u32, height: u32) -> usize
    {
        3 * width as usize * height as usize / 2
    }

    fn init_black(width: u32, height: u32, storage: &mut [C])
    {
        assert!(storage.len() == <Self as ColorMode<C>>::channel_data_size(width, height));
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
        use std::mem::transmute_copy;

        let start_subpixel = (3 * (x + width * y)) as usize;

        let mut pixel = [Channel::from_i32(0, 0, 2); 3];
        pixel.copy_from_slice(&holder[start_subpixel..][..3]);

        ColorYuv::new_yuv(pixel[0], pixel[1], pixel[2])
    }

    #[inline]
    fn put_pixel(holder: &mut [C], width: u32, height: u32, x: u32, y: u32, pixel: <Self as ColorMode<C>>::Pixel) {
        let pixels = width as usize * height as usize;
        let offset_y = x + width * y;
        let offset_c = (x / 2) + width * (y / 2);
        get_y_mut(holder, pixels)[offset_y as usize] = pixel.y;
        get_u_mut(holder, pixels)[offset_c as usize] = pixel.u;
        get_v_mut(holder, pixels)[offset_c as usize] = pixel.v;
    }
}
