use std::ops::{Deref, DerefMut};

use super::{ColorMode};
use super::super::Channel;
use super::super::colorspace::ColorYUV as ColorYuv;


#[derive(Clone)]
pub struct Yuv420p;

// #[derive(Clone)]
// pub struct Yuv420pHolder<C> where C: Channel {
//     pixels: usize,
//     data: Box<[C]>,
// }

// impl Contiguous for Yuv420pHolder<u8> {
//     fn raw_bytes(&self) -> &[u8] {
//         &self.data[..]
//     }

//     fn raw_bytes_mut(&mut self) -> &mut [u8] {
//         &mut self.data[..]
//     }
// }

// impl<C> PlaneHolder<C> for Yuv420pHolder<C> where C: Channel {
//     fn new(width: u32, height: u32, data: &[C]) -> Self {
//         let mut pixels_u = width as usize * height as usize;
//         if data.len() != 3 * pixels_u / 2 {
//             panic!("Invalid data size");
//         }
//         Yuv420pHolder {
//             pixels: pixels_u,
//             data: Into::<Vec<_>>::into(data).into_boxed_slice(),
//         }
//     }

//     fn new_black(width: u32, height: u32) -> Self {
//         let mut pixels_u = width as usize * height as usize;
//         let mut subpixels = 3 * pixels_u / 2;

//         let mut pixels = vec![Channel::min_value(); subpixels].into_boxed_slice();
//         for px in pixels[pixels_u..].iter_mut() {
//             *px = Channel::from_i32(1, 0, 2);
//         }

//         Yuv420pHolder {
//             pixels: pixels_u,
//             data: pixels,
//         }
//     }

//     fn get(&self, idx: usize) -> &[C] {
//         match idx {
//             0 => self.get_y(),
//             1 => self.get_u(),
//             2 => self.get_v(),
//             _ => panic!("channel out of range"),
//         }
//     }

//     fn get_mut(&mut self, idx: usize) -> &mut [C] {
//         match idx {
//             0 => self.get_y_mut(),
//             1 => self.get_u_mut(),
//             2 => self.get_v_mut(),
//             _ => panic!("channel out of range"),
//         }
//     }
// }


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
