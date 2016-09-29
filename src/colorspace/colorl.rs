use super::{Channel, Pixel};

#[derive(Debug, Copy)]
pub struct ColorL<T> {
    pub l: T,
}

impl<T: Clone> Clone for ColorL<T> {
    fn clone(&self) -> ColorL<T> {
        ColorL { l: self.l.clone() }
    }
}

impl<T: Channel> ColorL<T> {
    pub fn new_l(l: T) -> ColorL<T> {
        ColorL { l: l }
    }
}

impl<T> Pixel for ColorL<T> where T: Channel+Copy {
    type Channel = T;

    fn white() -> Self {
        ColorL { l: Channel::max_value() }
    }

    fn black() -> Self {
        ColorL { l: Channel::min_value() }
    }

    fn luma(&self) -> ColorL<Self::Channel> {
        ColorL::new_l(self.l)
    }
}