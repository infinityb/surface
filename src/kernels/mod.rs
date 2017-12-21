use super::{Channel, Pixel};
use super::{
    ColorL,
    ColorLA,
    ColorRGB,
    ColorRGBA,
    ColorYUV,
};

mod kernel_3x3;

pub use self::kernel_3x3::{
    Sobel3x3,
    Average3x3,
};


pub trait Kernel3x3<C>
    where
        C: Channel,
{
    fn execute(data: &[[C; 9]], &mut [C]);
}


// pub trait Kernel4x4<C>
//     where
//         C: Channel,
// {
//     fn execute(data: &[[C; 9]], &mut [C]);
// }

// pub trait Kernel5x5<C>
//     where
//         C: Channel,
// {
//     fn execute(data: &[[C; 9]], &mut [C]);
// }


// pub trait Kernel<C: Channel, P: Pixel<Channel=C>>
// {
//     type Region;
    
//     fn execute(subpixels: &Self::Region, out: &mut P);
// }

// pub struct Average3x3New;

// impl<C: Channel> Kernel<C, ColorL<C>> for Average3x3New
// {
//     type Region = [ColorL<C>; 9];

//     fn execute(pixels: &[ColorL<C>; 9], out: &mut ColorL<C>) {
//         let mut acc: i32 = 0;

//         for px in pixels.iter() {
//             acc += Channel::to_i32(&subpixels[0].l, 0, 0xFFFF);
//         }
        
//         *out.l = Channel::from_i32(acc / 9, 0, 0xFFFF);
//     }
// }

// impl<C: Channel> Kernel<C, ColorLA<C>> for Average3x3New
// {
//     type Region = [ColorLA<C>; 9];

//     fn execute(subpixels: &[ColorLA<C>; 9], out: &mut ColorLA<C>) {
//         let mut acc_l: i32 = 0;
//         let mut acc_a: i32 = 0;

//         for px in pixels.iter() {
//             acc_l += Channel::to_i32(&subpixels[0].l, 0, 0xFFFF);
//             acc_a += Channel::to_i32(&subpixels[0].a, 0, 0xFFFF);
//         }
        
//         *out.l = Channel::from_i32(acc_l / 9, 0, 0xFFFF);
//         *out.a = Channel::from_i32(acc_a / 9, 0, 0xFFFF);
//     }
// }

// impl<C: Channel> Kernel<C, ColorRGB<C>> for Average3x3New
// {
//     type Region = [ColorRGB<C>; 9];

//     fn execute(subpixels: &[ColorRGB<C>; 9], out: &mut ColorRGB<C>) {
//         let mut acc_r: i32 = 0;
//         let mut acc_g: i32 = 0;
//         let mut acc_b: i32 = 0;

//         for px in pixels.iter() {
//             acc_r += Channel::to_i32(&subpixels[0].r, 0, 0xFFFF);
//             acc_g += Channel::to_i32(&subpixels[0].g, 0, 0xFFFF);
//             acc_b += Channel::to_i32(&subpixels[0].b, 0, 0xFFFF);
//         }
        
//         *out.r = Channel::from_i32(acc_r / 9, 0, 0xFFFF);
//         *out.g = Channel::from_i32(acc_g / 9, 0, 0xFFFF);
//         *out.b = Channel::from_i32(acc_b / 9, 0, 0xFFFF);
//     }
// }

// impl<C: Channel> Kernel<C, ColorRGBA<C>> for Average3x3New
// {
//     type Region = [ColorRGBA<C>; 9];

//     fn execute(subpixels: &[ColorRGBA<C>; 9], out: &mut ColorRGBA<C>) {
//         let mut acc_r: i32 = 0;
//         let mut acc_g: i32 = 0;
//         let mut acc_b: i32 = 0;
//         let mut acc_a: i32 = 0;

//         for px in pixels.iter() {
//             acc_r += Channel::to_i32(&subpixels[0].r, 0, 0xFFFF);
//             acc_g += Channel::to_i32(&subpixels[0].g, 0, 0xFFFF);
//             acc_b += Channel::to_i32(&subpixels[0].b, 0, 0xFFFF);
//             acc_a += Channel::to_i32(&subpixels[0].a, 0, 0xFFFF);
//         }
        
//         *out.r = Channel::from_i32(acc_r / 9, 0, 0xFFFF);
//         *out.g = Channel::from_i32(acc_g / 9, 0, 0xFFFF);
//         *out.b = Channel::from_i32(acc_b / 9, 0, 0xFFFF);
//         *out.a = Channel::from_i32(acc_a / 9, 0, 0xFFFF);
//     }
// }
