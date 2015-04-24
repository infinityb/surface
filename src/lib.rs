extern crate num;
extern crate netpbm;

pub use self::colorrgba::{Channel, ColorRGBA};
pub use self::surface::{Surface, SubsurfaceIterator};
pub use self::surfacefactory::SurfaceFactory;
pub use self::surfaceiterator::SurfaceIterator;

mod colorrgba;
mod surface;
mod surfacefactory;
mod surfaceiterator;
mod netpbm_loader;
