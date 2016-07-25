use std::marker::PhantomData;

use super::colorspace::{Colorspace, ColorYUV as ColorYuv, ColorL};
use super::Channel;


pub trait Kernel3x3<S> where S: Colorspace {
    fn execute(data: &[S; 9]) -> S;
}


pub trait PlaneHolder<C> {
    fn get(&self, idx: usize) -> &[C];

    fn get_mut(&mut self, idx: usize) -> &mut [C];
}

impl<C> PlaneHolder<C> for [Box<[C]>; 1] where C: Channel {
    fn get(&self, idx: usize) -> &[C] {
        let plane = &self[idx];
        &plane[..]
    }

    fn get_mut(&mut self, idx: usize) -> &mut [C] {
        let plane = &mut self[idx];
        &mut plane[..]
    }
}

impl<C> PlaneHolder<C> for [Box<[C]>; 3] where C: Channel {
    fn get(&self, idx: usize) -> &[C] {
        let plane = &self[idx];
        &plane[..]
    }

    fn get_mut(&mut self, idx: usize) -> &mut [C] {
        let plane = &mut self[idx];
        &mut plane[..]
    }
}

pub trait ColorMode<C> where C: Channel {
    type Pixel: Colorspace;
    type Holder: PlaneHolder<C>;

    fn create_planes(width: u32, height: u32, data: &[C]) -> Self::Holder;

    fn create_planes_black(width: u32, height: u32) -> Self::Holder;

    fn put_pixel(holder: &mut Self::Holder, width: u32, height: u32, x: u32, y: u32, pixel: Self::Pixel);

    fn get_pixel(holder: &Self::Holder, width: u32, height: u32, x: u32, y: u32) -> Self::Pixel;
}

// // Can't represent with current internals.
//
// pub struct RgbInterleaved;
//
// pub struct RgbInterleavedHolder<C> {
//     data: Box<[(C, C, C)]>,
// }
//
//
// pub struct RgbPlanar;

pub struct Yuv420p;

pub struct Yuv420pHolder<C> {
    pixels: usize,
    data: Box<[C]>,
}

impl<C> PlaneHolder<C> for Yuv420pHolder<C> {
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

impl<C> Yuv420pHolder<C> {
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

pub struct Luma;

impl<C> ColorMode<C> for Luma where C: Channel {
    type Pixel = ColorL<C>;
    type Holder = [Box<[C]>; 1];

    fn create_planes(width: u32, height: u32, data: &[C]) -> Self::Holder {
        let mut pixels = width as usize * height as usize;
        if data.len() != pixels {
            panic!("Invalid data size");
        }

        [Into::<Vec<_>>::into(data).into_boxed_slice()]
    }

    fn create_planes_black(width: u32, height: u32) -> Self::Holder {
        let length = width as usize * height as usize;

        [vec![Channel::min_value(); length].into_boxed_slice()]
    }

    fn put_pixel(holder: &mut Self::Holder, width: u32, height: u32, x: u32, y: u32, pixel: Self::Pixel) {
        let offset_y = x + width * y;
        holder[0][offset_y as usize] = pixel.l;
    }

    fn get_pixel(holder: &Self::Holder, width: u32, height: u32, x: u32, y: u32) -> Self::Pixel {
        let offset_y = x + width * y;
        ColorL::new_l(holder[0][offset_y as usize])
    }
}

pub struct PlanarSurface<H, M, C> {
    width: u32,
    height: u32,
    planes: H,
    _mode_marker: PhantomData<M>,
    _channel_marker: PhantomData<C>,
}

impl<H, M, C> PlanarSurface<H, M, C>
    where
        H: PlaneHolder<C>,
        M: ColorMode<C, Holder=H>,
        C: Channel,
{
    pub fn new(width: u32, height: u32, data: &[C]) -> PlanarSurface<H, M, C> {
        PlanarSurface {
            width: width,
            height: height,
            planes: <M as ColorMode<C>>::create_planes(width, height, data),
            _mode_marker: PhantomData,
            _channel_marker: PhantomData,
        }
    }

    pub fn new_black(width: u32, height: u32) -> PlanarSurface<H, M, C> {
        PlanarSurface {
            width: width,
            height: height,
            planes: <M as ColorMode<C>>::create_planes_black(width, height),
            _mode_marker: PhantomData,
            _channel_marker: PhantomData,
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> M::Pixel {
        <M as ColorMode<_>>::get_pixel(&self.planes, self.width, self.height, x, y)
    }

    pub fn put_pixel(&mut self, x: u32, y: u32, val: M::Pixel) {
        <M as ColorMode<_>>::put_pixel(&mut self.planes, self.width, self.height, x, y, val)
    }

    pub fn run_kernel_3x3<K>(&self, kernel: &K)
        -> PlanarSurface<H, M, C>
        where
            K: Kernel3x3<<M as ColorMode<C>>::Pixel>
    {
        let mut out = PlanarSurface::new_black(self.width, self.height);

        let mut data: [<M as ColorMode<C>>::Pixel; 9] = [Colorspace::black(); 9];
        for y in 0..self.height {
            for x in 0..self.width {
                surf_3x3_get(self, &mut data, x, y);
                <K as Kernel3x3<_>>::execute(&data);
            }
        }

        out
    }
}

impl<C> PlanarSurface<Yuv420pHolder<C>, Yuv420p, C> where C: Channel {
    pub fn extract_luma(&self) -> PlanarSurface<[Box<[C]>; 1], Luma, C> {
        PlanarSurface::new(self.width, self.height, self.planes.get_y())
    }
}

// impl<C> PlanarSurface<RgbHolder<C>, Rgb, C> where C: Channel {
//     pub fn extract_luma(&self) -> PlanarSurface<[Box<[C]>; 1], Luma, C> {
//         PlanarSurface::new(self.width, self.height, self.planes.get_y())
//     }
// }


//// -------------

#[inline]
fn surf_3x3_get<H, M, C>(
    inp: &PlanarSurface<H, M, C>,
    data: &mut [<M as ColorMode<C>>::Pixel; 9],
    x_pos: u32,
    y_pos: u32,
)
    where
        H: PlaneHolder<C>,
        M: ColorMode<C, Holder=H>,
        C: Channel,
{
    *data = [
        <M as ColorMode<C>>::get_pixel(&inp.planes, inp.width, inp.height, x_pos - 1, y_pos - 1),
        <M as ColorMode<C>>::get_pixel(&inp.planes, inp.width, inp.height, x_pos + 0, y_pos - 1),
        <M as ColorMode<C>>::get_pixel(&inp.planes, inp.width, inp.height, x_pos + 1, y_pos - 1),

        <M as ColorMode<C>>::get_pixel(&inp.planes, inp.width, inp.height, x_pos - 1, y_pos + 0),
        <M as ColorMode<C>>::get_pixel(&inp.planes, inp.width, inp.height, x_pos + 0, y_pos + 0),
        <M as ColorMode<C>>::get_pixel(&inp.planes, inp.width, inp.height, x_pos + 1, y_pos + 0),

        <M as ColorMode<C>>::get_pixel(&inp.planes, inp.width, inp.height, x_pos - 1, y_pos + 1),
        <M as ColorMode<C>>::get_pixel(&inp.planes, inp.width, inp.height, x_pos + 0, y_pos + 1),
        <M as ColorMode<C>>::get_pixel(&inp.planes, inp.width, inp.height, x_pos + 1, y_pos + 1),
    ];
}