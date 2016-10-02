mod kernel_3x3;
mod kernel_3x3_l8;

pub use self::kernel_3x3::{
    Sobel3x3,
    Average3x3,
};

pub use self::kernel_3x3_l8::{
    Luma8Sobel3x3,
	Luma8Average3x3,
};