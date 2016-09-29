use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use super::colorspace::{Pixel, ColorYUV as ColorYuv, ColorL};
use super::Channel;

mod yuv420;
mod luma;

pub use self::yuv420::{Yuv420p};
pub use self::luma::{Luma};

pub trait Kernel3x3<S> where P: Pixel {
    fn execute(data: &[P; 9]) -> P;
}

pub trait ColorMode<C>
    where
        C: Channel
{
    type Pixel: Pixel;

    // type PixelRef;

    fn channel_data_size(width: u32, height: u32) -> usize;

    fn init_black(width: u32, height: u32, storage: &mut [C]);

    fn get_pixel(storage: &[C], width: u32, height: u32, x: u32, y: u32) -> Self::Pixel;

    fn put_pixel(holder: &mut [C], width: u32, height: u32, x: u32, y: u32, pixel: <Self as ColorMode<C>>::Pixel);
}

#[derive(Clone)]
pub struct PlanarSurface<M, C, S>
    where
        M: ColorMode<C>,
        C: Channel,
        S: Deref<Target=[C]>,
{
    width: u32,
    height: u32,
    storage: S,
    _mode_marker: PhantomData<M>,
    _channel_marker: PhantomData<C>,
}

impl<M, C, S> PlanarSurface<M, C, S>
    where
        M: ColorMode<C, Pixel=C>,
        C: Channel,
        S: Deref<Target=[C]>,
{
    pub fn new(width: u32, height: u32, storage: S) -> PlanarSurface<M, C, S> {
        PlanarSurface {
            width: width,
            height: height,
            storage: storage,
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

    pub fn pixels(&self) -> Pixels<M, C, S> {
        Pixels::new(self)
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> M::Pixel {
        <M as ColorMode<C>>::get_pixel(&self.storage, self.width, self.height, x, y)
    }

    pub fn to_owned(&self) -> PlanarSurface<M, C, Box<[C]>> {
        PlanarSurface::new(self.width, self.height, copy_to_boxed_slice(&self.storage))
    }

    pub fn into_storage(self) -> S {
        self.storage
    }

    pub fn run_kernel_3x3<S2, K>(&self, kernel: &K, output: &mut PlanarSurface<M, C, S2>)
        where
            K: Kernel3x3<<M as ColorMode<C>>::Pixel>,
            S2: Deref<Target=[C]> + DerefMut,
    {
        assert_eq!(self.width, output.width);
        assert_eq!(self.height, output.height);

        let mut data: [<M as ColorMode<C>>::Pixel; 9] = [Pixel::black(); 9];
        for y in 1..(self.height - 1) {
            for x in 1..(self.width - 1) {
                surf_3x3_get(self, &mut data, x, y);
                PlanarSurface::put_pixel(output, x, y, <K as Kernel3x3<_>>::execute(&data));
            }
        }
    }
}

impl<M, C, S> PlanarSurface<M, C, S>
    where
        M: ColorMode<C>,
        C: Channel,
        S: Deref<Target=[C]> + DerefMut,
{
    pub fn put_pixel(&mut self, x: u32, y: u32, val: M::Pixel) {
        <M as ColorMode<_>>::put_pixel(&mut self.storage, self.width, self.height, x, y, val)
    }
}

impl<M, S> PlanarSurface<M, u8, S>
    where
        M: ColorMode<u8>,
        S: Deref<Target=[u8]>
{
    pub fn raw_bytes(&self) -> &[u8] {
        &self.storage[..]
    }
}

impl<M, S> PlanarSurface<M, u8, S>
    where
        M: ColorMode<u8>,
        S: Deref<Target=[u8]> + DerefMut
{
    pub fn raw_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.storage[..]
    }
}

impl<C, S> PlanarSurface<Yuv420p, C, S>
    where
        C: Channel,
        S: Deref<Target=[C]>,
{
    pub fn extract_luma<'a>(&'a self) -> PlanarSurface<Luma, C, &'a [C]> {
        let pixels = self.width as usize * self.width as usize;
        let luma = yuv420::get_y(&self.storage, pixels);
        PlanarSurface::new(self.width, self.height, luma)
    }
}

impl<S> PlanarSurface<Luma, u8, S>
    where
        S: Deref<Target=[u8]>,
{
    pub fn run_luma8_kernel_3x3<S2>(&self, kernel: fn(pixels: &[u8; 9]) -> u8, output: &mut PlanarSurface<Luma, u8, S2>)
        where
            S2: Deref<Target=[u8]> + DerefMut
    {
        use std::mem::transmute_copy;

        assert_eq!(self.width, output.width);
        assert_eq!(self.height, output.height);

        let mut data_pix: [<Luma as ColorMode<u8>>::Pixel; 9] = [Pixel::black(); 9];
        let mut data: [u8; 9] = [0; 9];
        for y in 0..self.height {
            for x in 0..self.width {
                surf_3x3_get(self, &mut data_pix, x, y);
                data = unsafe { transmute_copy(&data_pix) };
                output.put_pixel(x, y, ColorL::new_l(kernel(&data)));
            }
        }
    }
}

// TODO: bound-check elision
pub struct Pixels<'a, M, C, S>
    where
        M: ColorMode<C> + 'a,
        C: Channel + 'a,
        S: Deref<Target=[C]> + 'a,
{
    surface: &'a PlanarSurface<M, C, S>,
    x_pos: u32,
    y_pos: u32,
}

impl<'a, M, C, S> Pixels<'a, M, C, S>
    where
        M: ColorMode<C>,
        C: Channel,
        S: Deref<Target=[C]>,
{
    fn new(surface: &PlanarSurface<M, C, S>) -> Pixels<M, C, S> {
        Pixels {
            surface: surface,
            x_pos: 0,
            y_pos: 0,
        }
    }
}

impl<'a, M, C, S> Iterator for Pixels<'a, M, C, S>
    where
        M: ColorMode<C>,
        C: Channel,
        S: Deref<Target=[C]> + 'a,
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
fn surf_3x3_get<M, C, S>(
    inp: &PlanarSurface<M, C, S>,
    data: &mut [<M as ColorMode<C>>::Pixel; 9],
    x_pos: u32,
    y_pos: u32,
)
    where
        M: ColorMode<C>,
        C: Channel,
        S: Deref<Target=[C]>,
{
    *data = [
        <M as ColorMode<C>>::get_pixel(&inp.storage, inp.width, inp.height, x_pos - 1, y_pos - 1),
        <M as ColorMode<C>>::get_pixel(&inp.storage, inp.width, inp.height, x_pos + 0, y_pos - 1),
        <M as ColorMode<C>>::get_pixel(&inp.storage, inp.width, inp.height, x_pos + 1, y_pos - 1),

        <M as ColorMode<C>>::get_pixel(&inp.storage, inp.width, inp.height, x_pos - 1, y_pos + 0),
        <M as ColorMode<C>>::get_pixel(&inp.storage, inp.width, inp.height, x_pos + 0, y_pos + 0),
        <M as ColorMode<C>>::get_pixel(&inp.storage, inp.width, inp.height, x_pos + 1, y_pos + 0),

        <M as ColorMode<C>>::get_pixel(&inp.storage, inp.width, inp.height, x_pos - 1, y_pos + 1),
        <M as ColorMode<C>>::get_pixel(&inp.storage, inp.width, inp.height, x_pos + 0, y_pos + 1),
        <M as ColorMode<C>>::get_pixel(&inp.storage, inp.width, inp.height, x_pos + 1, y_pos + 1),
    ];
}

fn copy_to_boxed_slice<C>(data: &[C]) -> Box<[C]> where C: Copy {
    Into::<Vec<C>>::into(data).into_boxed_slice()
}