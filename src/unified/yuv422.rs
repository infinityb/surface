use std::ops::{Deref, DerefMut};

use super::{Format, PlanarFormat};
use super::super::Channel;
use super::super::colorspace::ColorYUV as ColorYuv;


/// Packed YUV 4:2:2
#[derive(Clone)]
pub struct Yuv422;

#[inline]
fn get_yuv422_yuv<C>(data: &[C], (w, h): (u32, u32), (x, y): (u32, u32)) -> (&C, &C, &C) {
    assert!(x < w);
    assert!(y < h);
    let data_base = data.as_ptr();
    let (w, h) = (w as isize, h as isize);
    let (x, y) = (x as isize, y as isize);
    unsafe {
        (
            &*data_base.offset(2 * (w * y + x)),
            &*data_base.offset(2 * ((w * y + x) & !1) + 1),
            &*data_base.offset(2 * ((w * y + x) & !1) + 3),
        )
    }
}

#[inline]
fn get_yuv422_yuv_mut<C>(data: &mut [C], (w, h): (u32, u32), (x, y): (u32, u32)) -> (&mut C, &mut C, &mut C) {
    assert!(x < w);
    assert!(y < h);
    let data_base = data.as_mut_ptr();
    let (w, h) = (w as isize, h as isize);
    let (x, y) = (x as isize, y as isize);
    unsafe {
        (
            &mut *data_base.offset(2 * (w * y + x)),
            &mut *data_base.offset(2 * ((w * y + x) & !1) + 1),
            &mut *data_base.offset(2 * ((w * y + x) & !1) + 3),
        )
    }
}

impl<C> Format<C> for Yuv422 where C: Channel {
    type Pixel = ColorYuv<C>;

    fn channel_data_size(width: u32, height: u32) -> usize
    {
        2 * width as usize * height as usize / 2
    }

    fn init_black(width: u32, height: u32, storage: &mut [C])
    {
        assert!(storage.len() == <Self as Format<C>>::channel_data_size(width, height));
        let pixels = width as usize * height as usize;

        let luma_min = <C as Channel>::from_i32(0, 0, 2);
        let chroma_neutral = <C as Channel>::from_i32(1, 0, 2);

        let mut iter = storage.iter_mut();
        for _ in 0..(pixels * 2) {
            if let Some(y) = iter.next() {
                *y = luma_min;
            }
            if let Some(u) = iter.next() {
                *u = chroma_neutral;
            }
            if let Some(v) = iter.next() {
                *v = chroma_neutral;
            } else {
                panic!("invalid storage");
            }
        }
    }

    #[inline]
    fn get_pixel(holder: &[C], width: u32, height: u32, x: u32, y: u32) -> Self::Pixel {
        let (y, u, v) = get_yuv422_yuv(holder, (width, height), (x, y));
        ColorYuv::new_yuv(*y, *u, *v)
    }

    #[inline]
    fn put_pixel(holder: &mut [C], width: u32, height: u32, x: u32, y: u32, pixel: <Self as Format<C>>::Pixel) {
        // TODO: clobbers colliding pixels. Does it matter?
        let (y, u, v) = get_yuv422_yuv_mut(holder, (width, height), (x, y));
        *y = pixel.y;
        *u = pixel.u;
        *v = pixel.v;
    }
}


/// Planar YUV 4:2:2
#[derive(Clone)]
pub struct Yuv422p;

#[inline]
fn get_yuv422p_yuv<C>(data: &[C], (w, h): (u32, u32), (x, y): (u32, u32)) -> (&C, &C, &C) {
    assert!(x < w);
    assert!(y < h);
    let data_base = data.as_ptr();
    let (w, h) = (w as isize, h as isize);
    let (x, y) = (x as isize, y as isize);
    let u_offset = w * h;
    let v_offset = 3 * (w * h) / 2;
    unsafe {
        (
            &*data_base.offset(w * y + x),
            &*data_base.offset(u_offset + ((w * y + x) / 2)),
            &*data_base.offset(v_offset + ((w * y + x) / 2)),
        )
    }
}


#[inline]
fn get_yuv422p_yuv_mut<C>(data: &mut [C], (w, h): (u32, u32), (x, y): (u32, u32)) -> (&mut C, &mut C, &mut C) {
    assert!(x < w);
    assert!(y < h);
    let data_base = data.as_mut_ptr();
    let (w, h) = (w as isize, h as isize);
    let (x, y) = (x as isize, y as isize);
    let u_offset = w * h;
    let v_offset = 3 * (w * h) / 2;
    unsafe {
        (
            &mut *data_base.offset(w * y + x),
            &mut *data_base.offset(u_offset + ((w * y + x) / 2)),
            &mut *data_base.offset(v_offset + ((w * y + x) / 2)),
        )
    }
}

impl<C> Format<C> for Yuv422p where C: Channel {
    type Pixel = ColorYuv<C>;

    fn channel_data_size(width: u32, height: u32) -> usize
    {
        2 * width as usize * height as usize
    }

    fn init_black(width: u32, height: u32, storage: &mut [C])
    {
        assert!(storage.len() == <Self as Format<C>>::channel_data_size(width, height));
        let pixels = width as usize * height as usize;

        let luma_min = <C as Channel>::from_i32(0, 0, 2);
        let chroma_neutral = <C as Channel>::from_i32(1, 0, 2);

        let mut iter = storage.iter_mut();
        for _ in 0..pixels {
            *iter.next().unwrap() = luma_min;
        }
        for _ in 0..pixels {
            *iter.next().unwrap() = chroma_neutral;
        }
    }

    #[inline]
    fn get_pixel(holder: &[C], width: u32, height: u32, x: u32, y: u32) -> Self::Pixel {
        let (y, u, v) = get_yuv422p_yuv(holder, (width, height), (x, y));
        ColorYuv::new_yuv(*y, *u, *v)
    }

    #[inline]
    fn put_pixel(holder: &mut [C], width: u32, height: u32, x: u32, y: u32, pixel: <Self as Format<C>>::Pixel) {
        // TODO: clobbers colliding pixels. Does it matter?
        let (y, u, v) = get_yuv422p_yuv_mut(holder, (width, height), (x, y));
        *y = pixel.y;
        *u = pixel.u;
        *v = pixel.v;
    }
}

impl<'a, C> PlanarFormat<'a, C> for Yuv422p
    where
        C: Channel + 'a
{
    type Planes = (&'a [C], &'a [C], &'a [C]);
    type PlanesMut = (&'a mut [C], &'a mut [C], &'a mut [C]);

    fn get_planes(data: &'a [C], (w, h): (u32, u32)) -> Self::Planes {
        let (w, h) = (w as usize, h as usize);
        let (y_plane, rest) = data.split_at(w * h);
        let (u_plane, v_plane) = rest.split_at(w * h / 2);
        (y_plane, u_plane, v_plane)
    }

    fn get_planes_mut(data: &'a mut [C], (w, h): (u32, u32)) -> Self::PlanesMut {
        let (w, h) = (w as usize, h as usize);
        let (y_plane, rest) = data.split_at_mut(w * h);
        let (u_plane, v_plane) = rest.split_at_mut(w * h / 2);
        (y_plane, u_plane, v_plane)
    }
}
