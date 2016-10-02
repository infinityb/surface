use super::super::unified::Kernel3x3;
use ::{ColorL, Luma};

pub struct Luma8Sobel3x3;

impl Kernel3x3<u8, ColorL<u8>> for Luma8Sobel3x3
{
    fn execute(pixels: &[ColorL<u8>; 9]) -> ColorL<u8> {
        ColorL {
            l: l8_sobel_3x3(&[
                pixels[0].l,
                pixels[1].l,
                pixels[2].l,
                pixels[3].l,
                pixels[4].l,
                pixels[5].l,
                pixels[6].l,
                pixels[7].l,
                pixels[8].l,
            ])
        }
    }
}

pub fn l8_sobel_3x3(pixels: &[u8; 9]) -> u8 {
    let mut acc_x = 0;
    let mut acc_y = 0;

    // acc_x
    acc_x -= 1 * pixels[0 + 3 * 0] as i32;      // (x=0, y=0)
    acc_x += 1 * pixels[2 + 3 * 0] as i32;      // (x=2, y=0)

    acc_x -= 2 * pixels[0 + 3 * 1] as i32;      // (x=0, y=1)
    acc_x += 2 * pixels[2 + 3 * 1] as i32;      // (x=2, y=1)

    acc_x -= 1 * pixels[0 + 3 * 2] as i32;      // (x=0, y=2)
    acc_x += 1 * pixels[2 + 3 * 2] as i32;      // (x=2, y=2)

    // acc_y
    acc_y -= 1 * pixels[0 + 3 * 0] as i32;      // (x=0, y=0)
    acc_y -= 2 * pixels[1 + 3 * 0] as i32;      // (x=1, y=0)
    acc_y -= 1 * pixels[2 + 3 * 0] as i32;      // (x=2, y=0)

    acc_y += 1 * pixels[0 + 3 * 2] as i32;      // (x=0, y=2)
    acc_y += 2 * pixels[1 + 3 * 2] as i32;      // (x=1, y=2)
    acc_y += 1 * pixels[2 + 3 * 2] as i32;      // (x=2, y=2)


    let acc_s = ((acc_y * acc_y + acc_x * acc_x)) as f32;
    clamp(acc_s.sqrt().round() as i32, 0x00, 0xFF) as u8
}

pub struct Luma8Average3x3;

impl Kernel3x3<u8, ColorL<u8>> for Luma8Average3x3
{
    fn execute(pixels: &[ColorL<u8>; 9]) -> ColorL<u8> {
        let mut acc = 0;

        for px in pixels.iter() {
            acc += px.l as i16;
        }

        ColorL { l: (acc / 9) as u8 }
    }
}

fn clamp<T: Ord>(value: T, min_value: T, max_value: T) -> T {
    use std::cmp::{min, max};
    
    max(min(value, max_value), min_value)
}

