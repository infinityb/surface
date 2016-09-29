use super::super::{Channel, Pixel};
use super::Format;
use super::super::colorspace::ColorL;

#[derive(Clone)]
pub struct Luma;

impl<C> Format<C> for Luma where C: Channel {
    type Pixel = ColorL<C>;

    fn channel_data_size(width: u32, height: u32) -> usize
    {
        width as usize * height as usize
    }

    fn init_black(width: u32, height: u32, storage: &mut [C])
    {
        assert!(storage.len() == <Self as Format<C>>::channel_data_size(width, height));

        let luma_min = <C as Channel>::min_value();

        for ch in storage.iter_mut() {
            *ch = luma_min;
        }
    }

    fn get_pixel(storage: &[C], width: u32, height: u32, x: u32, y: u32) -> Self::Pixel {
        let offset_y = x + width * y;
        ColorL::new_l(storage[offset_y as usize])
    }
    
    #[inline]
    fn put_pixel(storage: &mut [C], width: u32, height: u32, x: u32, y: u32, pixel: <Self as Format<C>>::Pixel) {
        let offset_y = x + width * y;
        storage[offset_y as usize] = pixel.l;
    }
}
