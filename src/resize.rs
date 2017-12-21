use std::ops::{Deref, DerefMut};

use super::{Surface, Channel, Format};

pub fn resize_nearest<F, C, SI, SO>(input: &Surface<F, C, SI>, output: &mut Surface<F, C, SO>)
    where
        F: Format<C>,
        C: Channel,
        SI: Deref<Target=[C]>,
        SO: Deref<Target=[C]> + DerefMut, 
{
    let (input_width, input_height) = (input.width(), input.height());
    let (output_width, output_height) = (output.width(), output.height());

    for y in 0..output_height {
        let sy = ((y * input_height) as f64 / output_height as f64).round() as u32;
        for x in 0..output_width {
            let sx = ((x * input_width) as f64 / output_width as f64).round() as u32;
            output.put_pixel(x, y, input.get_pixel(sx, sy));
        }
    }
}


// pub struct Surface<M, C, S>
//     where
//         M: Format<C>,
//         C: Channel,
//         S: Deref<Target=[C]>,