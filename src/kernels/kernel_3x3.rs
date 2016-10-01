use super::super::unified::Kernel3x3;
use super::super::{Pixel, Channel, ColorL};


pub struct Sobel3x3;


// TODO: this needs to be generic over something else...
//       fixed-array Pixel?  Then we could iterate the channels.
impl<C> Kernel3x3<C, ColorL<C>> for Sobel3x3
    where
        C: Channel,
{
    fn execute(pixels: &[ColorL<C>; 9]) -> ColorL<C> {
        let mut acc_x = 0;
        let mut acc_y = 0;

        // acc_x
        acc_x -= 1 * Channel::to_i32(&pixels[0 + 3 * 0].l, 0, 0xFFFF);      // (x=0, y=0)
        acc_x += 1 * Channel::to_i32(&pixels[2 + 3 * 0].l, 0, 0xFFFF);      // (x=2, y=0)

        acc_x -= 2 * Channel::to_i32(&pixels[0 + 3 * 1].l, 0, 0xFFFF);      // (x=0, y=1)
        acc_x += 2 * Channel::to_i32(&pixels[2 + 3 * 1].l, 0, 0xFFFF);      // (x=2, y=1)

        acc_x -= 1 * Channel::to_i32(&pixels[0 + 3 * 2].l, 0, 0xFFFF);      // (x=0, y=2)
        acc_x += 1 * Channel::to_i32(&pixels[2 + 3 * 2].l, 0, 0xFFFF);      // (x=2, y=2)

        // acc_y
        acc_y -= 1 * Channel::to_i32(&pixels[0 + 3 * 0].l, 0, 0xFFFF);      // (x=0, y=0)
        acc_y -= 2 * Channel::to_i32(&pixels[1 + 3 * 0].l, 0, 0xFFFF);      // (x=1, y=0)
        acc_y -= 1 * Channel::to_i32(&pixels[2 + 3 * 0].l, 0, 0xFFFF);      // (x=2, y=0)

        acc_y += 1 * Channel::to_i32(&pixels[0 + 3 * 2].l, 0, 0xFFFF);      // (x=0, y=2)
        acc_y += 2 * Channel::to_i32(&pixels[1 + 3 * 2].l, 0, 0xFFFF);      // (x=1, y=2)
        acc_y += 1 * Channel::to_i32(&pixels[2 + 3 * 2].l, 0, 0xFFFF);      // (x=2, y=2)

        let acc_x = acc_x as f64;
        let acc_y = acc_y as f64;

        let value = clamp((acc_y * acc_y + acc_x * acc_x).sqrt().round() as i32, 0x00, 0xFFFF);

        ColorL { l: Channel::from_i32(value, 0, 0xFFFF) }
    }
}

pub struct Average3x3;

impl<C> Kernel3x3<C, ColorL<C>> for Average3x3
    where
        C: Channel,
{
    fn execute(pixels: &[ColorL<C>; 9]) -> ColorL<C> {
        let mut acc = 0;

        for px in pixels.iter() {
            acc +=  Channel::to_i32(&px.l, 0, 0xFFFF);
        }

        ColorL { l: Channel::from_i32(acc / 9, 0, 0xFFFF) }
    }
}

fn clamp<T: Ord>(value: T, min_value: T, max_value: T) -> T {
    use std::cmp::{min, max};

    max(min(value, max_value), min_value)
}
