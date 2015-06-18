use netpbm::{PpmPixel, PpmLoadResult, PpmLoadError, FromPpm};

use super::colorrgba::{ColorRGBA, Channel};
use super::surface::Surface;

impl FromPpm for Surface<u8> {
    fn from_ppm(width: u32, height: u32, depth: u32,
                pixels: &mut Iterator<Item=PpmLoadResult<PpmPixel>>
               ) -> PpmLoadResult<Surface<u8>> {

        // Check Channel::max_depth();
        if let Some(max_depth) = <u8 as Channel>::max_depth() {
            if max_depth < depth {
                return Err(PpmLoadError::OverflowError);
            }
        }

        let mut surface: Surface<u8> = Surface::new(
            width as usize, height as usize, ColorRGBA::black());

        for (idx, pixel) in pixels.enumerate() {
            let PpmPixel(r, g, b) = try!(pixel);
            surface[idx] = ColorRGBA::new_rgb(r as u8, g as u8, b as u8);
        }

        Ok(surface)
    }
}

