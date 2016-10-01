
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

pub fn l8_average_3x3(pixels: &[u8; 9]) -> u8 {
    let mut acc = 0;

    for px in pixels.iter() {
        acc += *px as i16;
    }

    (acc / 9) as u8
}

fn clamp<T: Ord>(value: T, min_value: T, max_value: T) -> T {
    use std::cmp::{min, max};
    
    max(min(value, max_value), min_value)
}
