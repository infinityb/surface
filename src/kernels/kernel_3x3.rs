use super::super::planar::Kernel3x3;
use super::super::{Colorspace, Channel};

fn channel_luma<S: Colorspace>(cs: &S) -> i32 {
    cs.luma() as i32
}

pub struct Sobel3x3;


// TODO: this needs to be generic over something else...
//       fixed-array Colorspace?  Then we could iterate the channels.
impl<P> Kernel3x3<P> for Sobel3x3 where P: Colorspace {
    fn execute(pixels: &[P; 9]) -> P {
        let mut acc_x = 0;
        let mut acc_y = 0;

        // acc_x
        acc_x -= 1 * pixels[0 + 3 * 0].to_i32(0, 0xFFFF);      // (x=0, y=0)
        acc_x += 1 * pixels[2 + 3 * 0].to_i32(0, 0xFFFF);      // (x=2, y=0)

        acc_x -= 2 * pixels[0 + 3 * 1].to_i32(0, 0xFFFF);      // (x=0, y=1)
        acc_x += 2 * pixels[2 + 3 * 1].to_i32(0, 0xFFFF);      // (x=2, y=1)

        acc_x -= 1 * pixels[0 + 3 * 2].to_i32(0, 0xFFFF);      // (x=0, y=2)
        acc_x += 1 * pixels[2 + 3 * 2].to_i32(0, 0xFFFF);      // (x=2, y=2)

        // acc_y
        acc_y -= 1 * pixels[0 + 3 * 0].to_i32(0, 0xFFFF);      // (x=0, y=0)
        acc_y -= 2 * pixels[1 + 3 * 0].to_i32(0, 0xFFFF);      // (x=1, y=0)
        acc_y -= 1 * pixels[2 + 3 * 0].to_i32(0, 0xFFFF);      // (x=2, y=0)

        acc_y += 1 * pixels[0 + 3 * 2].to_i32(0, 0xFFFF);      // (x=0, y=2)
        acc_y += 2 * pixels[1 + 3 * 2].to_i32(0, 0xFFFF);      // (x=1, y=2)
        acc_y += 1 * pixels[2 + 3 * 2].to_i32(0, 0xFFFF);      // (x=2, y=2)


        let acc_s = ((acc_y * acc_y + acc_x * acc_x)) as f32;
        let value = clamp(acc_s.sqrt().round() as i32, 0x00, 0xFFFF);

        Channel::from_i32(value, 0, 0xFFFF)
    }
}

pub struct Average3x3;

impl<P> Kernel3x3<P> for Average3x3 where P: Colorspace {
    fn execute(data: &[P; 9]) -> P {
        let mut acc = 0;

        for px in pixels.iter() {
            acc += px.to_i32(0, 0xFFFF);
        }

        Channel::from_i32(acc / 9, 0, 0xFFFF)
    }
}
