//! YUV420 only for now.

struct SurfaceRing {
    stride: usize,
    offset: usize,
    storage: Box<[u8]>,
}

impl SurfaceRing {
    pub fn new(width: u32, height: u32) -> SurfaceRing {
        let (width_u, height_u) = (width as usize, height as usize);
        vec![0; width_u * height_u * Cs::??]
    }
}

struct StreamSurface {
    width: u32,
    height: u32,
    ring_storage: SurfaceRing,
}


trait Kernel<T> {
    fn size(&self) -> (u32, u32);

    fn operate(&self, pixels: &[u8]) -> T;
}


struct SobelKernel;

impl Kernel for SobelKernel {
    fn size(&self) -> (u32, u32) {
        (3, 3)
    }

    fn operate(&self, pixels: &[u8]) -> T {
        use std::mem::transmute_copy;

        let pixels: [u8; 36] = unsafe { transmute_copy(pixels) };

        //
    }
}