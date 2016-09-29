use super::{ColorMode};
use super::super::Channel;
use super::super::colorspace::ColorL;

#[derive(Clone)]
pub struct Luma;

// impl<C> PlaneHolder<C> for LumaHolder<C> where C: Channel {
//     fn new(width: u32, height: u32, data: &[C]) -> Self {
//         let mut pixels = width as usize * height as usize;
//         if data.len() != pixels {
//             panic!("Invalid data size");
//         }

//         LumaHolder {
//             data: Into::<Vec<_>>::into(data).into_boxed_slice()
//         }
//     }

//     fn new_black(width: u32, height: u32) -> Self {
//         let length = width as usize * height as usize;

//         LumaHolder {
//             data: vec![Channel::min_value(); length].into_boxed_slice(),
//         }
//     }

//     fn get(&self, idx: usize) -> &[C] {
//         match idx {
//             0 => &self.data[..],
//             _ => panic!("channel out of range"),
//         }
//     }

//     fn get_mut(&mut self, idx: usize) -> &mut [C] {
//         match idx {
//             0 => &mut self.data[..],
//             _ => panic!("channel out of range"),
//         }
//     }
// }

impl<C> ColorMode<C> for Luma where C: Channel {
    type Pixel = ColorL<C>;

    fn channel_data_size(width: u32, height: u32) -> usize
    {
        width as usize * height as usize
    }

    fn init_black(width: u32, height: u32, storage: &mut [C])
    {
        assert!(storage.len() == <Self as ColorMode<C>>::channel_data_size(width, height));

        let luma_min = <C as Channel>::from_i32(0, 0, 2);

        for ch in storage.iter_mut() {
            *ch = luma_min;
        }
    }

    fn get_pixel(storage: &[C], width: u32, height: u32, x: u32, y: u32) -> Self::Pixel {
        let offset_y = x + width * y;
        ColorL::new_l(storage[offset_y as usize])
    }
    
    #[inline]
    fn put_pixel(storage: &mut [C], width: u32, height: u32, x: u32, y: u32, pixel: <Self as ColorMode<C>>::Pixel) {
        let offset_y = x + width * y;
        storage[offset_y as usize] = pixel.l;
    }
}
