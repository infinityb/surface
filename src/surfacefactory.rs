use super::{Colorspace, Surface};


pub struct SurfaceFactory<CS> {
    pub width: usize,
    pub height: usize,
    pub x_off: usize,
    pub y_off: usize,
    pub background: CS,
}


impl<CS> SurfaceFactory<CS> where CS: Colorspace {
    pub fn new(width: usize, height: usize, x_off: usize, y_off: usize,
               background: CS) -> SurfaceFactory<CS> {
        SurfaceFactory {
            width: width,
            height: height,
            x_off: x_off,
            y_off: y_off,
            background: background
        }
    }

    #[allow(dead_code)]
    pub fn create(&self) -> Surface<CS> {
        Surface::with_offset(self.width, self.height, self.x_off, self.y_off, self.background)
    }
}
