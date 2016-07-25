mod kernel_3x3;
mod kernel_3x3_l8;

pub use self::kernel_3x3::{
    Sobel3x3,
    Average3x3,
};

pub use self::kernel_3x3_l8::{
    l8_sobel_3x3,
    l8_average_3x3,
}