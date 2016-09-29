use std::ops::{Deref, DerefMut};

use super::{Format};
use super::super::Channel;
use super::super::colorspace::{
    ColorRGBA as ColorRgba,
    ColorRGB as ColorRgb,
};


#[inline(always)]
fn get_plane_offset(width: u32, height: u32, plane: u32) -> usize {
    let (width, height, plane) = (width as usize, height as usize, plane as usize);
    width * height * plane
}

#[inline(always)]
fn get_offset_in_plane(width: u32, x: u32, y: u32) -> usize {
    let (x, y, width) = (x as usize, y as usize, width as usize);
    x + width * y
}

#[derive(Clone)]
pub struct Rgb;

impl<C> Format<C> for Rgb where C: Channel {
    type Pixel = ColorRgb<C>;

    fn channel_data_size(width: u32, height: u32) -> usize
    {
        3 * width as usize * height as usize
    }

    fn init_black(width: u32, height: u32, storage: &mut [C])
    {
        assert!(storage.len() == <Self as Format<C>>::channel_data_size(width, height));

        let ch_min = <C as Channel>::min_value();
        for ch in storage.iter_mut() {
            *ch = ch_min;
        }
    }

    #[inline]
    fn get_pixel(storage: &[C], width: u32, _height: u32, x: u32, y: u32) -> Self::Pixel
    {
        let offset = 3 * get_offset_in_plane(x, y, width);
        let px = &storage[offset..];
        ColorRgb::new_rgb(px[0], px[1], px[2])
    }

    #[inline]
    fn put_pixel(storage: &mut [C], width: u32, height: u32, x: u32, y: u32, pixel: <Self as Format<C>>::Pixel)
    {   
        let offset = 3 * get_offset_in_plane(x, y, width);
        let px = &mut storage[offset..];
        px[0] = pixel.r;
        px[1] = pixel.g;
        px[2] = pixel.b;
    }
}


#[derive(Clone)]
pub struct Rgba;

impl<C> Format<C> for Rgba where C: Channel {
    type Pixel = ColorRgba<C>;

    fn channel_data_size(width: u32, height: u32) -> usize
    {
        4 * width as usize * height as usize
    }

    fn init_black(width: u32, height: u32, storage: &mut [C])
    {
        assert!(storage.len() == <Self as Format<C>>::channel_data_size(width, height));

        let ch_min = <C as Channel>::min_value();
        for ch in storage.iter_mut() {
            *ch = ch_min;
        }
    }

    #[inline]
    fn get_pixel(storage: &[C], width: u32, _height: u32, x: u32, y: u32) -> Self::Pixel
    {
        let offset = 4 * get_offset_in_plane(x, y, width);
        let px = &storage[offset..];
        ColorRgba::new_rgba(px[0], px[1], px[2], px[3])
    }

    #[inline]
    fn put_pixel(storage: &mut [C], width: u32, height: u32, x: u32, y: u32, pixel: <Self as Format<C>>::Pixel)
    {   
        let offset = 4 * get_offset_in_plane(x, y, width);
        let px = &mut storage[offset..];
        px[0] = pixel.r;
        px[1] = pixel.g;
        px[2] = pixel.b;
        px[3] = pixel.a;
    }
}


#[derive(Clone)]
pub struct RgbPlanar;

impl<C> Format<C> for RgbPlanar where C: Channel {
    type Pixel = ColorRgb<C>;

    fn channel_data_size(width: u32, height: u32) -> usize
    {
        3 * width as usize * height as usize
    }

    fn init_black(width: u32, height: u32, storage: &mut [C])
    {
        assert!(storage.len() == <Self as Format<C>>::channel_data_size(width, height));

        let ch_min = <C as Channel>::min_value();
        for ch in storage.iter_mut() {
            *ch = ch_min;
        }
    }

    #[inline]
    fn get_pixel(storage: &[C], width: u32, height: u32, x: u32, y: u32) -> Self::Pixel
    {
        let inplane = get_offset_in_plane(width, x, y);
        let r_off = get_plane_offset(width, height, 0) + inplane;
        let g_off = get_plane_offset(width, height, 1) + inplane;
        let b_off = get_plane_offset(width, height, 2) + inplane;
        ColorRgb {
            r: storage[r_off],
            g: storage[g_off],
            b: storage[b_off],
        }
    }

    #[inline]
    fn put_pixel(storage: &mut [C], width: u32, height: u32, x: u32, y: u32, pixel: <Self as Format<C>>::Pixel)
    {
        let inplane = get_offset_in_plane(width, x, y);
        let r_off = get_plane_offset(width, height, 0) + inplane;
        let g_off = get_plane_offset(width, height, 1) + inplane;
        let b_off = get_plane_offset(width, height, 2) + inplane;
        storage[r_off] = pixel.r;
        storage[g_off] = pixel.g;
        storage[b_off] = pixel.b;
    }
}


#[derive(Clone)]
pub struct RgbaPlanar;

impl<C> Format<C> for RgbaPlanar where C: Channel {
    type Pixel = ColorRgba<C>;

    fn channel_data_size(width: u32, height: u32) -> usize
    {
        4 * width as usize * height as usize
    }

    fn init_black(width: u32, height: u32, storage: &mut [C])
    {
        assert!(storage.len() == <Self as Format<C>>::channel_data_size(width, height));

        let ch_min = <C as Channel>::min_value();
        for ch in storage.iter_mut() {
            *ch = ch_min;
        }
    }

    #[inline]
    fn get_pixel(storage: &[C], width: u32, height: u32, x: u32, y: u32) -> Self::Pixel
    {
        let inplane = get_offset_in_plane(width, x, y);
        let r_off = get_plane_offset(width, height, 0) + inplane;
        let g_off = get_plane_offset(width, height, 1) + inplane;
        let b_off = get_plane_offset(width, height, 2) + inplane;
        let a_off = get_plane_offset(width, height, 3) + inplane;
        ColorRgba {
            r: storage[r_off],
            g: storage[g_off],
            b: storage[b_off],
            a: storage[a_off],
        }
    }

    #[inline]
    fn put_pixel(storage: &mut [C], width: u32, height: u32, x: u32, y: u32, pixel: <Self as Format<C>>::Pixel)
    {
        let inplane = get_offset_in_plane(width, x, y);
        let r_off = get_plane_offset(width, height, 0) + inplane;
        let g_off = get_plane_offset(width, height, 1) + inplane;
        let b_off = get_plane_offset(width, height, 2) + inplane;
        let a_off = get_plane_offset(width, height, 3) + inplane;
        storage[r_off] = pixel.r;
        storage[g_off] = pixel.g;
        storage[b_off] = pixel.b;
        storage[a_off] = pixel.a;
    }
}
