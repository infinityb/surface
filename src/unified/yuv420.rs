use std::ops::{Deref, DerefMut};

use super::{Format, PlanarFormat};
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

impl<C> Format<C> for Yuv420p where C: Channel {
    type Pixel = ColorYuv<C>;

    fn channel_data_size(width: u32, height: u32) -> usize
    {
        3 * width as usize * height as usize / 2
    }

    fn init_black(width: u32, height: u32, storage: &mut [C])
    {
        assert_eq!(width % 2, 0);
        assert_eq!(height % 2, 0);
        
        assert!(storage.len() == <Self as Format<C>>::channel_data_size(width, height));
        let pixels = width as usize * width as usize;

        let luma_min = <C as Channel>::from_i32(0, 0, 2);
        let chroma_neutral = <C as Channel>::from_i32(1, 0, 2);

        let mut pxiter = storage.iter_mut();
        for _ in 0..pixels {
            if let Some(px) = pxiter.next() {
                *px = luma_min;
            }
        }
        while let Some(px) = pxiter.next() {
            *px = chroma_neutral;
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
    fn put_pixel(holder: &mut [C], width: u32, height: u32, x: u32, y: u32, pixel: <Self as Format<C>>::Pixel) {
        let pixels = width as usize * height as usize;
        let offset_y = x + width * y;
        let offset_c = (x / 2) + width * (y / 2);
        get_y_mut(holder, pixels)[offset_y as usize] = pixel.y;
        get_u_mut(holder, pixels)[offset_c as usize] = pixel.u;
        get_v_mut(holder, pixels)[offset_c as usize] = pixel.v;
    }
}

impl<'a, C> PlanarFormat<'a, C> for Yuv420p
    where
        C: Channel + 'a
{
    type Planes = (&'a [C], &'a [C], &'a [C]);
    type PlanesMut = (&'a mut [C], &'a mut [C], &'a mut [C]);

    fn get_planes(data: &'a [C], (w, h): (u32, u32)) -> Self::Planes {
        let (w, h) = (w as usize, h as usize);
        let (y_plane, rest) = data.split_at(w * h);
        let (u_plane, v_plane) = rest.split_at(w * h / 4);
        (y_plane, u_plane, v_plane)
    }

    fn get_planes_mut(data: &'a mut [C], (w, h): (u32, u32)) -> Self::PlanesMut {
        let (w, h) = (w as usize, h as usize);
        let (y_plane, rest) = data.split_at_mut(w * h);
        let (u_plane, v_plane) = rest.split_at_mut(w * h / 4);
        (y_plane, u_plane, v_plane)
    }
}
