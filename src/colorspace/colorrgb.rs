use super::{Colorspace, Channel};

#[derive(Debug, Copy)]
pub struct ColorRGB<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl<T: Clone> Clone for ColorRGB<T> {
    fn clone(&self) -> ColorRGB<T> {
        ColorRGB {
            r: self.r.clone(),
            g: self.g.clone(),
            b: self.b.clone(),
        }
    }
}

impl<T: Channel> ColorRGB<T> {
    #[allow(dead_code)]
    pub fn new_rgb(r: T, g: T, b: T) -> ColorRGB<T> {
        ColorRGB { r: r, g: g, b: b }
    }
}

impl<T> Colorspace for ColorRGB<T> where T: Channel+Copy {
    type Channel = T;

    fn white() -> Self {
        ColorRGB::new_rgb(
            Channel::max_value(),
            Channel::max_value(),
            Channel::max_value())
    }

    fn black() -> Self {
        ColorRGB::new_rgb(
            Channel::min_value(),
            Channel::min_value(),
            Channel::min_value())
    }

    fn luma(&self) -> T {
        let (r, g, b) = (
            Channel::to_i32(&self.r, 0, 0xFF),
            Channel::to_i32(&self.g, 0, 0xFF),
            Channel::to_i32(&self.b, 0, 0xFF));
        
        let luma_val = (19595*r + 38470*g + 7471*b + 1<<15) >> 16;

        Channel::from_i32(luma_val, 0, 0xFF)
    }
}