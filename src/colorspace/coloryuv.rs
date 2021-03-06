use super::{Channel, Pixel, ColorL};

#[derive(Debug, Copy)]
pub struct ColorYUV<T> {
    pub y: T,
    pub u: T,
    pub v: T,
}

impl<T: Clone> Clone for ColorYUV<T> {
    fn clone(&self) -> ColorYUV<T> {
        ColorYUV {
            y: self.y.clone(),
            u: self.u.clone(),
            v: self.v.clone(),
        }
    }
}

impl<T: Channel> ColorYUV<T> {
    #[allow(dead_code)]
    pub fn new_yuv(y: T, u: T, v: T) -> ColorYUV<T> {
        ColorYUV { y: y, u: u, v: v }
    }
}

impl<T> Pixel for ColorYUV<T> where T: Channel+Copy {
    type Channel = T;
    
    fn white() -> Self {
        ColorYUV::new_yuv(
            Channel::max_value(),
            Channel::max_value(),
            Channel::max_value())
    }

    fn black() -> Self {
        ColorYUV::new_yuv(
            Channel::min_value(),
            Channel::min_value(),
            Channel::min_value())
    }

    fn luma(&self) -> ColorL<Self::Channel> {
        ColorL::new_l(self.y)
    }
}
