extern crate num;
extern crate netpbm;

pub use self::colorspace::{Channel, Colorspace};
pub use self::colorspace::{
	ColorL,
	ColorLA,
	ColorRGB,
	ColorRGBA,
};

pub use self::surface::{Surface, SubsurfaceIterator};
pub use self::surfacefactory::SurfaceFactory;
pub use self::surfaceiterator::SurfaceIterator;

mod colorspace;
mod surface;
mod surfacefactory;
mod surfaceiterator;

pub mod netpbm_loader;
