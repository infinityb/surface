use netpbm::{PpmPixel, PpmLoadResult, PpmLoadError, FromPpm};

use super::{ColorRGBA, Channel, Colorspace};
use super::Surface;

impl FromPpm for Surface {
    fn from_ppm(width: u32, height: u32, depth: u32,
                pixels: &mut Iterator<Item=PpmLoadResult<PpmPixel>>
               ) -> PpmLoadResult<Surface> {

        // Check Channel::max_depth();
        if let Some(max_depth) = <u8 as Channel>::max_depth() {
            if max_depth < depth {
                return Err(PpmLoadError::OverflowError);
            }
        }

        let mut surface: Surface = Surface::new(
            width as usize, height as usize, ColorRGBA::black());

        for (idx, pixel) in pixels.enumerate() {
            let PpmPixel(r, g, b) = try!(pixel);
            surface[idx] = ColorRGBA::new_rgb(r as u8, g as u8, b as u8);
        }

        Ok(surface)
    }
}

