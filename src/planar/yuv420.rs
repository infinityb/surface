use super::{Contiguous, PlaneHolder, ColorMode};
use super::super::Channel;
use super::super::colorspace::ColorYUV as ColorYuv;

#[derive(Clone)]
pub struct Yuv420p;

#[derive(Clone)]
pub struct Yuv420pHolder<C> where C: Copy {
    pixels: usize,
    data: Box<[C]>,
}

impl Contiguous for Yuv420pHolder<u8> {
    fn raw_bytes(&self) -> &[u8] {
        &self.data[..]
    }

    fn raw_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.data[..]
    }
}

impl<C> PlaneHolder<C> for Yuv420pHolder<C> where C: Copy {
    fn get(&self, idx: usize) -> &[C] {
        match idx {
            0 => self.get_y(),
            1 => self.get_u(),
            2 => self.get_v(),
            _ => panic!("channel out of range"),
        }
    }

    fn get_mut(&mut self, idx: usize) -> &mut [C] {
        match idx {
            0 => self.get_y_mut(),
            1 => self.get_u_mut(),
            2 => self.get_v_mut(),
            _ => panic!("channel out of range"),
        }
    }
}

impl<C> Yuv420pHolder<C> where C: Copy {
    #[inline]
    pub fn get_y(&self) -> &[C] {
        &self.data[..self.pixels]
    }

    #[inline]
    pub fn get_u(&self) -> &[C] {
        &self.data[self.pixels..][..self.pixels / 4]
    }

    #[inline]
    pub fn get_v(&self) -> &[C] {
        &self.data[self.pixels..][self.pixels / 4..][..self.pixels / 4]
    }

    #[inline]
    pub fn get_y_mut(&mut self) -> &mut [C] {
        &mut self.data[..self.pixels]
    }

    #[inline]
    pub fn get_u_mut(&mut self) -> &mut [C] {
        &mut self.data[self.pixels..][..self.pixels / 4]
    }

    #[inline]
    pub fn get_v_mut(&mut self) -> &mut [C] {
        &mut self.data[self.pixels..][self.pixels / 4..][..self.pixels / 4]
    }
}

impl<C> ColorMode<C> for Yuv420p where C: Channel {
    type Pixel = ColorYuv<C>;
    type Holder = Yuv420pHolder<C>;

    fn create_planes(width: u32, height: u32, data: &[C]) -> Self::Holder {
        let mut pixels_u = width as usize * height as usize;
        if data.len() != 3 * pixels_u / 2 {
            panic!("Invalid data size");
        }
        Yuv420pHolder {
            pixels: pixels_u,
            data: Into::<Vec<_>>::into(data).into_boxed_slice(),
        }
    }

    fn create_planes_black(width: u32, height: u32) -> Self::Holder {
        let mut pixels_u = width as usize * height as usize;
        let mut subpixels = 3 * pixels_u / 2;

        let mut pixels = vec![Channel::min_value(); subpixels].into_boxed_slice();
        for px in pixels[pixels_u..].iter_mut() {
            *px = Channel::from_i32(1, 0, 2);
        }

        Yuv420pHolder {
            pixels: pixels_u,
            data: pixels,
        }
    }

    #[inline]
    fn put_pixel(holder: &mut Self::Holder, width: u32, height: u32, x: u32, y: u32, pixel: Self::Pixel) {
        let offset_y = x + width * y;
        let offset_c = (x / 2) + width * (y / 2);
        holder.get_y_mut()[offset_y as usize] = pixel.y;
        holder.get_u_mut()[offset_c as usize] = pixel.u;
        holder.get_v_mut()[offset_c as usize] = pixel.v;
    }

    #[inline]
    fn get_pixel(holder: &Self::Holder, width: u32, height: u32, x: u32, y: u32) -> Self::Pixel {
        let offset_y = x + width * y;
        let offset_c = (x / 2) + width * (y / 2);
        let y = holder.get_y()[offset_y as usize];
        let u = holder.get_u()[offset_c as usize];
        let v = holder.get_v()[offset_c as usize];
        ColorYuv::new_yuv(y, u, v)
    }
}
