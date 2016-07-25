use super::{Contiguous, PlaneHolder, ColorMode};
use super::super::Channel;
use super::super::colorspace::ColorL;

pub struct Luma;

#[derive(Clone)]
pub struct LumaHolder<C> where C: Copy {
    data: Box<[C]>,
}

impl Contiguous for LumaHolder<u8> {
    fn raw_bytes(&self) -> &[u8] {
        &self.data[..]
    }

    fn raw_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.data[..]
    }
}

impl<C> PlaneHolder<C> for LumaHolder<C> where C: Copy {
    fn get(&self, idx: usize) -> &[C] {
        match idx {
            0 => &self.data[..],
            _ => panic!("channel out of range"),
        }
    }

    fn get_mut(&mut self, idx: usize) -> &mut [C] {
        match idx {
            0 => &mut self.data[..],
            _ => panic!("channel out of range"),
        }
    }
}

impl<C> ColorMode<C> for Luma where C: Channel {
    type Pixel = ColorL<C>;
    type Holder = LumaHolder<C>;

    fn create_planes(width: u32, height: u32, data: &[C]) -> Self::Holder {
        let mut pixels = width as usize * height as usize;
        if data.len() != pixels {
            panic!("Invalid data size");
        }

        LumaHolder {
            data: Into::<Vec<_>>::into(data).into_boxed_slice()
        }
    }

    fn create_planes_black(width: u32, height: u32) -> Self::Holder {
        let length = width as usize * height as usize;

        LumaHolder {
            data: vec![Channel::min_value(); length].into_boxed_slice(),
        }
    }

    fn put_pixel(holder: &mut Self::Holder, width: u32, height: u32, x: u32, y: u32, pixel: Self::Pixel) {
        let offset_y = x + width * y;
        holder.data[offset_y as usize] = pixel.l;
    }

    fn get_pixel(holder: &Self::Holder, width: u32, height: u32, x: u32, y: u32) -> Self::Pixel {
        let offset_y = x + width * y;
        ColorL::new_l(holder.data[offset_y as usize])
    }
}