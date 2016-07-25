use std::marker::PhantomData;

use super::colorspace::{Colorspace, ColorYUV as ColorYuv, ColorL};
use super::Channel;

mod yuv420;
mod luma;

pub use self::yuv420::{Yuv420p, Yuv420pHolder};
pub use self::luma::{Luma};


pub trait Contiguous {
    fn raw_bytes(&self) -> &[u8];

    fn raw_bytes_mut(&mut self) -> &mut [u8];
}

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

impl Contiguous for [Box<[u8]>; 1] {
    fn raw_bytes(&self) -> &[u8] {
        &self[0][..]
    }

    fn raw_bytes_mut(&mut self) -> &mut [u8] {
        &mut self[0][..]
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

pub struct PlanarSurface<M, C>
    where
        M: ColorMode<C>,
        C: Channel,
{
    width: u32,
    height: u32,
    planes: <M as ColorMode<C>>::Holder,
    _mode_marker: PhantomData<M>,
    _channel_marker: PhantomData<C>,
}

impl<M, C> PlanarSurface<M, C>
    where
        M: ColorMode<C>,
        C: Channel,
{
    pub fn new_black(width: u32, height: u32) -> PlanarSurface<M, C> {
        PlanarSurface {
            width: width,
            height: height,
            planes: <M as ColorMode<C>>::create_planes_black(width, height),
            _mode_marker: PhantomData,
            _channel_marker: PhantomData,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn pixels(&self) -> Pixels<M, C> {
        Pixels::new(self)
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> M::Pixel {
        <M as ColorMode<_>>::get_pixel(&self.planes, self.width, self.height, x, y)
    }

    pub fn put_pixel(&mut self, x: u32, y: u32, val: M::Pixel) {
        <M as ColorMode<_>>::put_pixel(&mut self.planes, self.width, self.height, x, y, val)
    }

    pub fn run_kernel_3x3<K>(&self, kernel: &K)
        -> PlanarSurface<M, C>
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

impl<M, C, H> PlanarSurface<M, C>
    where
        C: Channel,
        M: ColorMode<C, Holder=H>,
        H: PlaneHolder<C> + Contiguous + 'static,
{
    pub fn new(width: u32, height: u32, data: &[C]) -> PlanarSurface<M, C> {
        PlanarSurface {
            width: width,
            height: height,
            planes: <M as ColorMode<C>>::create_planes(width, height, data),
            _mode_marker: PhantomData,
            _channel_marker: PhantomData,
        }
    }

    pub fn raw_bytes(&self) -> &[u8] {
        self.planes.raw_bytes()
    }

    pub fn raw_bytes_mut(&mut self) -> &mut [u8] {
        self.planes.raw_bytes_mut()
    }
}

// TODO: generalize for all channels.  All pixels are copy-safe.
impl PlanarSurface<Yuv420p, u8> {
    pub fn extract_luma(&self) -> PlanarSurface<Luma, u8> {
        PlanarSurface::new(self.width, self.height, self.planes.get_y())
    }
}

impl PlanarSurface<Luma, u8> {
    pub fn run_luma8_kernel_3x3(&self, kernel: fn(pixels: &[u8; 9]) -> u8)
        -> PlanarSurface<Luma, u8>
    {
        use std::mem::transmute_copy;

        let mut out = PlanarSurface::new_black(self.width, self.height);

        let mut data_pix: [<Luma as ColorMode<u8>>::Pixel; 9] = [Colorspace::black(); 9];
        let mut data: [u8; 9] = [0; 9];
        for y in 0..self.height {
            for x in 0..self.width {
                surf_3x3_get(self, &mut data_pix, x, y);
                data = unsafe { transmute_copy(&data_pix) };
                out.put_pixel(x, y, ColorL::new_l(kernel(&data)));
            }
        }

        out
    }
}

// TODO: bound-check elision
pub struct Pixels<'a, M, C>
    where
        M: ColorMode<C> + 'a,
        C: Channel + 'a,
{
    surface: &'a PlanarSurface<M, C>,
    x_pos: u32,
    y_pos: u32,
}

impl<'a, M, C> Pixels<'a, M, C>
    where
        M: ColorMode<C>,
        C: Channel,
{
    fn new(surface: &PlanarSurface<M, C>) -> Pixels<M, C> {
        Pixels {
            surface: surface,
            x_pos: 0,
            y_pos: 0,
        }
    }
}

impl<'a, M, C> Iterator for Pixels<'a, M, C>
    where
        M: ColorMode<C>,
        C: Channel,
{
    type Item = M::Pixel;

    fn next(&mut self) -> Option<M::Pixel> {
        if self.surface.width <= self.x_pos && self.surface.height <= self.y_pos {
            return None;
        }
    
        let px = self.surface.get_pixel(self.x_pos, self.y_pos);
        self.x_pos += 1;

        if self.surface.width <= self.x_pos {
            self.x_pos = 0;
            self.y_pos += 1;
        }

        Some(px)
    }
}

// impl<C> PlanarSurface<RgbHolder<C>, Rgb, C> where C: Channel {
//     pub fn extract_luma(&self) -> PlanarSurface<[Box<[C]>; 1], Luma, C> {
//         PlanarSurface::new(self.width, self.height, self.planes.get_y())
//     }
// }


//// -------------

#[inline]
fn surf_3x3_get<M, C>(
    inp: &PlanarSurface<M, C>,
    data: &mut [<M as ColorMode<C>>::Pixel; 9],
    x_pos: u32,
    y_pos: u32,
)
    where
        M: ColorMode<C>,
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