use std::mem;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use super::kernels::{Kernel3x3};
use super::colorspace::{
    Pixel,
    ColorYUV as ColorYuv,
    ColorRGBA as ColorRgba,
    ColorL,
};
use super::Channel;

mod yuv420;
mod yuv422;
mod yuv444;
mod luma;
mod rgba;

pub use self::yuv420::{Yuv420p}; // Yuv420
pub use self::yuv422::{Yuv422, Yuv422p};
pub use self::yuv444::{Yuv444}; // Yuv444p

pub use self::luma::{Luma};
pub use self::rgba::{Rgb, RgbPlanar, Rgba, RgbaPlanar};

pub trait Format<C>
    where
        C: Channel
{
    type Pixel: Pixel<Channel=C>;

    // type PixelRef;

    fn channel_data_size(width: u32, height: u32) -> usize;

    fn init_black(width: u32, height: u32, storage: &mut [C]);

    fn get_pixel(storage: &[C], width: u32, height: u32, x: u32, y: u32) -> Self::Pixel;

    fn put_pixel(holder: &mut [C], width: u32, height: u32, x: u32, y: u32, pixel: <Self as Format<C>>::Pixel);
}

pub trait PlanarFormat<'a, C>: Format<C>
    where
        C: Channel + 'a
{
    type Planes: 'a;
    type PlanesMut: 'a;

    fn get_planes(data: &'a [C], wh: (u32, u32)) -> Self::Planes;

    fn get_planes_mut(data: &'a mut [C], wh: (u32, u32)) -> Self::PlanesMut;
}

#[derive(Clone)]
pub struct Surface<M, C, S>
    where
        M: Format<C>,
        C: Channel,
        S: Deref<Target=[C]>,
{
    width: u32,
    height: u32,
    storage: S,
    _mode_marker: PhantomData<M>,
    _channel_marker: PhantomData<C>,
}

pub fn surface_byte_size<M, C>(width: u32, height: u32) -> usize
where
    M: Format<C>,
    C: Channel,
{
    let length = <M as Format<C>>::channel_data_size(width, height);
    mem::size_of::<C>() * length
}

impl<M, C, S> Surface<M, C, S>
    where
        M: Format<C>,
        C: Channel,
        S: Deref<Target=[C]>,
{
    pub fn new(width: u32, height: u32, storage: S) -> Surface<M, C, S> {
        Surface {
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

    pub fn iter_pixels(&self) -> Pixels<M, C, S> {
        Pixels::new(self)
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> M::Pixel {
        <M as Format<C>>::get_pixel(&self.storage, self.width, self.height, x, y)
    }

    pub fn to_owned(&self) -> Surface<M, C, Box<[C]>> {
        Surface::new(self.width, self.height, copy_to_boxed_slice(&self.storage))
    }

    pub fn into_storage(self) -> S {
        self.storage
    }

    pub fn as_storage(&self) -> &S {
        &self.storage
    }

    // pub fn run_kernel<S2, K>(&self, kernel: &K, output: &mut Surface<M, C, S2>)
    //     where
    //         K: Kernel3x3<C>,
    //         S2: Deref<Target=[C]> + DerefMut,
    // {
    //     assert_eq!(self.width, output.width);
    //     assert_eq!(self.height, output.height);

    //     let mut data: [<M as Format<C>>::Pixel; 9] = [Pixel::black(); 9];
    //     for y in 1..(self.height - 1) {
    //         for x in 1..(self.width - 1) {
    //             surf_3x3_get(self, &mut data, x, y);
    //             Surface::put_pixel(output, x, y, <K as Kernel3x3<_>>::execute(&data), );
    //         }
    //     }
    // }
}

impl<M, C, S> Surface<M, C, S>
    where
        M: Format<C>,
        C: Channel,
        S: Deref<Target=[C]> + DerefMut,
{
    pub fn put_pixel(&mut self, x: u32, y: u32, val: M::Pixel) {
        <M as Format<C>>::put_pixel(&mut self.storage, self.width, self.height, x, y, val)
    }
}

impl<M, S> Surface<M, u8, S>
    where
        M: Format<u8>,
        S: Deref<Target=[u8]>
{
    pub fn raw_bytes(&self) -> &[u8] {
        &self.storage[..]
    }
}

impl<M, S> Surface<M, u8, S>
    where
        M: Format<u8>,
        S: Deref<Target=[u8]> + DerefMut
{
    pub fn raw_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.storage[..]
    }
}

impl<'a, M, C, S> Surface<M, C, S>
    where
        M: Format<C> + PlanarFormat<'a, C>,
        C: Channel + 'a,
        S: Deref<Target=[C]>
{
    pub fn get_planes(&'a self) -> <M as PlanarFormat<C>>::Planes {
        let size = (self.width, self.height);
        <M as PlanarFormat<C>>::get_planes(&self.storage, size)
    }

}

impl<'a, M, C, S> Surface<M, C, S>
    where
        M: Format<C> + PlanarFormat<'a, C>,
        C: Channel + 'a,
        S: Deref<Target=[C]> + DerefMut
{
    pub fn get_planes_mut(&'a mut self) -> <M as PlanarFormat<C>>::PlanesMut {
        let size = (self.width, self.height);
        <M as PlanarFormat<C>>::get_planes_mut(&mut self.storage, size)
    }
}

impl<M, C> Surface<M, C, Box<[C]>>
    where
        M: Format<C>,
        C: Channel,
{
    pub fn new_black(width: u32, height: u32) -> Surface<M, C, Box<[C]>> {
        let length = <M as Format<C>>::channel_data_size(width, height);
        let min = <C as Channel>::min_value();

        let mut storage = vec![min; length].into_boxed_slice();
        <M as Format<C>>::init_black(width, height, &mut storage);

        Surface {
            width: width,
            height: height,
            storage: storage,
            _mode_marker: PhantomData,
            _channel_marker: PhantomData,
        }
    }
}


pub fn extract_luma<M, C, S>(input: &Surface<M, C, S>)
-> Surface<Luma, C, Box<[C]>>
    where
        M: Format<C> + 'static,
        C: Channel,
        S: Deref<Target=[C]>,
{
    use std::any::TypeId;

    let mut out: Surface<Luma, C, Box<[C]>> = Surface::new_black(input.width, input.height);

    // wooo reflection -- probably optimised out entirely though?
    if TypeId::of::<M>() == TypeId::of::<Yuv420p>() {
        let pixels = input.width as usize * input.height as usize;
        let luma = yuv420::get_y(&input.storage, pixels);

        return Surface::new(input.width, input.height, luma).to_owned();
    }

    for y in 0..input.height {
        for x in 0..input.width {
            let px: <M as Format<C>>::Pixel = input.get_pixel(x, y);
            
            let px_luma: ColorL<C> = px.luma();

            out.put_pixel(x, y, px_luma);
        }
    }

    out
}


// impl<C, S> Surface<Yuv420p, C, S>
//     where
//         C: Channel,
//         S: Deref<Target=[C]>,
// {
//     // maybe stupid -- storage should be same? -- or cows
//     pub fn extract_luma2<'a>(&'a self) -> Surface<Luma, C, &'a [C]> {
//         let pixels = self.width as usize * self.width as usize;
//         let luma = yuv420::get_y(&self.storage, pixels);
//         Surface::new(self.width, self.height, luma)
//     }
// }

// impl<M, C, S> Surface<M, C, S>
//     where
//         M: Format<C>,
//         C: Channel,
//         S: Deref<Target=[C]>,
// {
//     pub fn extract_luma(&self) -> Surface<Luma, C, Box<[C]>> {
//         let mut out: Surface<Luma, C, Box<[C]>> = Surface::new_black(self.width, self.height);

//         for y in 0..self.height {
//             for x in 0..self.width {
//                 

//                 let px_luma: ColorL<C> = px.luma();
//                 out.put_pixel(x, y, px_luma);
//             }
//         }

//         out
//     }
// }


// impl<C, S> Surface<Rgba, C, S>
//     where
//         C: Channel,
//         S: Deref<Target=[C]>,
// {
//     pub fn extract_luma2<'a>(&'a self) -> Surface<Luma, C, Box<[C]>> {
//         let size = <Luma as Format<C>>::channel_data_size(self.width, self.height);
//         let min = Pixel::black();

//         let mut luma = vec![min; size].into_boxed_slice();
//         for (px, lpx) in self.storage.chunks(4).zip(luma.iter_mut()) {
//             *lpx = ColorRgba::new_rgba(px[0], px[1], px[2], px[3]).luma();
//         }
//         Surface::new(self.width, self.height, luma)
//     }
// }

impl<S> Surface<Luma, u8, S>
    where
        S: Deref<Target=[u8]>,
{
    pub fn run_luma8_kernel_3x3<S2>(&self, kernel: fn(pixels: &[u8; 9]) -> u8, output: &mut Surface<Luma, u8, S2>)
        where
            S2: Deref<Target=[u8]> + DerefMut
    {
        use std::mem::transmute_copy;

        assert_eq!(self.width, output.width);
        assert_eq!(self.height, output.height);

        let mut data_pix: [<Luma as Format<u8>>::Pixel; 9] = [Pixel::black(); 9];
        let mut data: [u8; 9] = [0; 9];
        for y in 1..(self.height - 1) {
            for x in 1..(self.width - 1) {
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
        M: Format<C> + 'a,
        C: Channel + 'a,
        S: Deref<Target=[C]> + 'a,
{
    surface: &'a Surface<M, C, S>,
    x_pos: u32,
    y_pos: u32,
}

impl<'a, M, C, S> Pixels<'a, M, C, S>
    where
        M: Format<C>,
        C: Channel,
        S: Deref<Target=[C]>,
{
    fn new(surface: &Surface<M, C, S>) -> Pixels<M, C, S> {
        Pixels {
            surface: surface,
            x_pos: 0,
            y_pos: 0,
        }
    }
}

impl<'a, M, C, S> Iterator for Pixels<'a, M, C, S>
    where
        M: Format<C>,
        C: Channel,
        S: Deref<Target=[C]> + 'a,
{
    type Item = M::Pixel;

    fn next(&mut self) -> Option<M::Pixel> {
        if self.surface.height <= self.y_pos {
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

// impl<C> Surface<RgbHolder<C>, Rgb, C> where C: Channel {
//     pub fn extract_luma(&self) -> Surface<[Box<[C]>; 1], Luma, C> {
//         Surface::new(self.width, self.height, self.planes.get_y())
//     }
// }


//// -------------

#[inline]
fn surf_3x3_get<M, C, S>(
    inp: &Surface<M, C, S>,
    data: &mut [<M as Format<C>>::Pixel; 9],
    x_pos: u32,
    y_pos: u32,
)
    where
        M: Format<C>,
        C: Channel,
        S: Deref<Target=[C]>,
{
    *data = [
        <M as Format<C>>::get_pixel(&inp.storage, inp.width, inp.height, x_pos - 1, y_pos - 1),
        <M as Format<C>>::get_pixel(&inp.storage, inp.width, inp.height, x_pos + 0, y_pos - 1),
        <M as Format<C>>::get_pixel(&inp.storage, inp.width, inp.height, x_pos + 1, y_pos - 1),

        <M as Format<C>>::get_pixel(&inp.storage, inp.width, inp.height, x_pos - 1, y_pos + 0),
        <M as Format<C>>::get_pixel(&inp.storage, inp.width, inp.height, x_pos + 0, y_pos + 0),
        <M as Format<C>>::get_pixel(&inp.storage, inp.width, inp.height, x_pos + 1, y_pos + 0),

        <M as Format<C>>::get_pixel(&inp.storage, inp.width, inp.height, x_pos - 1, y_pos + 1),
        <M as Format<C>>::get_pixel(&inp.storage, inp.width, inp.height, x_pos + 0, y_pos + 1),
        <M as Format<C>>::get_pixel(&inp.storage, inp.width, inp.height, x_pos + 1, y_pos + 1),
    ];
}

fn copy_to_boxed_slice<C>(data: &[C]) -> Box<[C]> where C: Copy {
    Into::<Vec<C>>::into(data).into_boxed_slice()
}
