use super::{Contiguous, PlaneHolder, ColorMode};
use super::super::Channel;
use super::super::colorspace::ColorL;

pub struct Luma;

impl<C> ColorMode<C> for Luma where C: Channel {
    type Pixel = ColorL<C>;
    type Holder = [Box<[C]>; 1];

    fn create_planes(width: u32, height: u32, data: &[C]) -> Self::Holder {
        let mut pixels = width as usize * height as usize;
        if data.len() != pixels {
            panic!("Invalid data size");
        }

        [Into::<Vec<_>>::into(data).into_boxed_slice()]
    }

    fn create_planes_black(width: u32, height: u32) -> Self::Holder {
        let length = width as usize * height as usize;

        [vec![Channel::min_value(); length].into_boxed_slice()]
    }

    fn put_pixel(holder: &mut Self::Holder, width: u32, height: u32, x: u32, y: u32, pixel: Self::Pixel) {
        let offset_y = x + width * y;
        holder[0][offset_y as usize] = pixel.l;
    }

    fn get_pixel(holder: &Self::Holder, width: u32, height: u32, x: u32, y: u32) -> Self::Pixel {
        let offset_y = x + width * y;
        ColorL::new_l(holder[0][offset_y as usize])
    }
}