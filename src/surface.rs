use std::cmp::min;
use std::iter::repeat;
use std::ops::{Index, IndexMut};

use super::{SurfaceFactory, Colorspace, ColorRGBA};

#[derive(Clone)]
pub struct Surface<CS=ColorRGBA<u8>> {
    pub width: usize,
    pub height: usize,
    pub x_off: usize,
    pub y_off: usize,
    pub background: CS,
    pub buffer: Vec<CS>,
}

impl<CS> Surface<CS> where CS: Colorspace {
    pub fn iter_pixels<'a>(&'a self) -> ::std::slice::Iter<'a, CS> {
        self.buffer.iter()
    }

    pub fn iter_pixels_mut<'a>(&'a mut self) -> ::std::slice::IterMut<'a, CS> {
        self.buffer.iter_mut()
    }
}

mod zigzag {
    pub fn to_idx(orig_size: (usize, usize), box_size: (usize, usize), coord: (usize, usize)) -> usize {
        let (width, height) = orig_size;
        let (box_width, box_height) = box_size;
        assert!(width % box_width == 0);
        assert!(height % box_height == 0);
        let box_length = box_width * box_height;
        let boxes_across = width / box_width;

        let (x, y) = coord;

        let (box_x, inner_x) = (x / box_width, x % box_width);
        let (box_y, inner_y) = (y / box_height, y % box_height);

        let mut idx = 0;
        idx += box_y * boxes_across + box_x;
        idx *= box_length;
        idx += inner_y * box_width + inner_x;
        idx
    }

    pub fn to_coord(orig_size: (usize, usize), box_size: (usize, usize), idx: usize) -> (usize, usize) {
        let (width, height) = orig_size;
        let (box_width, box_height) = box_size;
        assert!(width % box_width == 0);
        assert!(height % box_height == 0);
        let box_length = box_width * box_height;
        let boxes_across = width / box_width;

        let (box_idx, inner_idx) = (idx / box_length, idx % box_length);
        let (box_x, box_y) = (box_idx % boxes_across, box_idx / boxes_across);
        let (inner_x, inner_y) = (inner_idx % box_width, inner_idx / box_width);
        (box_x * box_width + inner_x, box_y * box_height + inner_y)
    }
}

mod xy {
    pub fn to_idx(orig_size: (usize, usize), coord: (usize, usize)) -> usize {
        let (width, height) = orig_size;
        let (x, y) = coord;
        if width <= x {
            panic!("`x` out of bounds (0 <= {} < {}", x, width);
        }
        if height <= y {
            panic!("`y` out of bounds (0 <= {} < {}", y, height);
        }
        width * y + x
    }

    pub fn to_coord(orig_size: (usize, usize), idx: usize) -> (usize, usize) {
        let (width, height) = orig_size;
        let (x, y) = (idx % width, idx / width);
        if width <= x {
            panic!("`x` out of bounds (0 <= {} < {}", x, width);
        }
        if height <= y {
            panic!("`y` out of bounds (0 <= {} < {}", y, height);
        }
        (x, y)
    }
}

impl<CS> Surface<CS> where CS: Colorspace {
    pub fn new(width: usize, height: usize, background: CS) -> Surface<CS> {
        Surface {
            width: width,
            height: height,
            x_off: 0,
            y_off: 0,
            background: background,
            buffer: repeat(background).take(width * height).collect()
        }
    }

    pub fn with_offset(width: usize, height: usize, x_off: usize, y_off: usize,
                       background: CS) -> Surface<CS> {
        Surface {
            width: width,
            height: height,
            x_off: x_off,
            y_off: y_off,
            background: background,
            buffer: repeat(background).take(width * height).collect()
        }
    }

    pub fn divide(&self, tile_width: usize, tile_height: usize) -> SubsurfaceIterator<CS> {
        SubsurfaceIterator {
            parent_width: self.width,
            parent_height: self.height,
            background: self.background,
            x_delta: tile_width,
            y_delta: tile_height,
            x_off: 0,
            y_off: 0,
        }
    }

    pub fn overrender_size(&self, tile_width: usize, tile_height: usize) -> (usize, usize) {
        let mut width = self.width;
        let width_partial_tile = width % tile_width;
        if width_partial_tile > 0 {
            width -= width_partial_tile;
            width += tile_width;
        }

        let mut height = self.height;
        let height_partial_tile = height % tile_height;
        if height_partial_tile > 0 {
            height -= height_partial_tile;
            height += tile_height;
        }

        (width, height)
    }

    pub fn merge(&mut self, tile: &Surface<CS>) {
        let x_len: usize = min(tile.width, self.width - tile.x_off);
        let y_len: usize = min(tile.height, self.height - tile.y_off);

        for src_y in 0..y_len {
            let dst_y = tile.y_off + src_y;
            for src_x in 0..x_len {
                let dst_x = tile.x_off + src_x;
                self[(dst_x, dst_y)] = tile[(src_x, src_y)]
            }
        }
    }

    #[inline]
    pub fn pixel_count(&self) -> usize {
        self.buffer.len()
    }

    #[inline]
    fn get_idx(&self, x: usize, y: usize) -> usize {
        xy::to_idx((self.width, self.height), (x, y))
    }

    pub fn coords(&self) -> CoordIterator {
        CoordIterator {
            index: 0,
            limit: self.width * self.height,
            width: self.width,
            height: self.height,
        }
    }

    pub fn coords_zigzag(&self) -> CoordIteratorZigZag {
        CoordIteratorZigZag {
            index: 0,
            limit: self.width * self.height,
            width: self.width,
            height: self.height,

            box_width: 128,
            box_height: 8,
        }
    }
}



impl<CS> Index<usize> for Surface<CS> where CS: Colorspace {
    type Output = CS;

    fn index<'a>(&'a self, index: usize) -> &'a CS {
        &self.buffer[index]
    }
}

impl<CS> IndexMut<usize> for Surface<CS> where CS: Colorspace {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut CS {
        &mut self.buffer[index]
    }
}

impl<CS> Index<(usize, usize)> for Surface<CS> where CS: Colorspace {
    type Output = CS;

    fn index<'a>(&'a self, index: (usize, usize)) -> &'a CS {
        let (x, y) = index;
        let idx = self.get_idx(x, y);
        &self.buffer[idx]
    }
}

impl<CS> IndexMut<(usize, usize)> for Surface<CS> where CS: Colorspace {
    fn index_mut<'a>(&'a mut self, index: (usize, usize)) -> &'a mut CS {
        let (x, y) = index;
        let idx = self.get_idx(x, y);
        &mut self.buffer[idx]
    }
}

#[derive(Debug)]
pub struct CoordIterator {
    index: usize,
    limit: usize,
    width: usize,
    height: usize,
}

impl Iterator for CoordIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<(usize, usize)> {
        if self.index < self.limit {
            let rv = Some((self.index % self.width, self.index / self.width));
            self.index += 1;
            rv
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct CoordIteratorZigZag {
    index: usize,
    limit: usize,
    width: usize,
    height: usize,

    box_width: usize,
    box_height: usize,
}

impl Iterator for CoordIteratorZigZag {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<(usize, usize)> {
        let orig_size = (self.width, self.height);
        let box_size = (self.box_width, self.box_height);

        if self.index < self.limit {
            let rv = zigzag::to_coord(orig_size, box_size, self.index);
            assert_eq!(zigzag::to_idx(orig_size, box_size, rv), self.index);
            self.index += 1;
            Some(rv)
        } else {
            None
        }
    }
}

pub struct SubsurfaceIterator<CS> {
    x_delta: usize,
    x_off: usize,
    y_delta: usize,
    y_off: usize,
    parent_width: usize,
    parent_height: usize,
    background: CS,
}

impl<CS> SubsurfaceIterator<CS> where CS: Colorspace {
    fn incr_tile(&mut self) {
        if self.x_off + self.x_delta < self.parent_width {
            self.x_off += self.x_delta;
        } else {
            self.x_off = 0;
            self.y_off += self.y_delta;
        }
    }

    fn current_tile(&self) -> Option<SurfaceFactory<CS>> {
        if self.x_off < self.parent_width && self.y_off < self.parent_height {
            Some(SurfaceFactory::new(
                self.x_delta,
                self.y_delta,
                self.x_off,
                self.y_off,
                self.background,
            ))
        } else {
            None
        }
    }
}

impl<CS> Iterator for SubsurfaceIterator<CS> where CS: Colorspace {
    type Item = SurfaceFactory<CS>;

    fn next(&mut self) -> Option<SurfaceFactory<CS>> {
        let tile = self.current_tile();
        self.incr_tile();
        tile
    }
}

#[test]
fn test_measurement() {
    let width = 800;
    let height = 600;
    let width_tile = 128;
    let height_tile = 8;

    let background: ColorRGBA<u8> = ColorRGBA::new_rgb(0, 0, 0);
    let surf: Surface = Surface::new(width, height, background);

    let mut total_pixels = 0;

    for tile_factory in surf.divide(width_tile, height_tile) {
        total_pixels += tile_factory.create().pixel_count();
    }

    let (or_width, or_height) = surf.overrender_size(width_tile, height_tile);

    assert_eq!(or_width * or_height, total_pixels);
}


#[test]
fn test_paint_it_red() {
    let width = 800;
    let height = 600;
    let width_tile = 128;
    let height_tile = 8;

    let background: ColorRGBA<u8> = ColorRGBA::new_rgb(0, 0, 0);
    let mut surf: Surface = Surface::new(width, height, background);

    for tile_factory in surf.divide(width_tile, height_tile) {
        let mut tile = tile_factory.create();
        for y in 0..tile.height {
            for x in 0..tile.width {
                tile[(x, y)] = ColorRGBA::new_rgb(255, 0, 0);
            }
        }
        for y in 0..tile.height {
            for x in 0..tile.width {
                assert_eq!(tile[(x, y)].r, 255);
                assert_eq!(tile[(x, y)].g, 0);
                assert_eq!(tile[(x, y)].b, 0);
            }
        }
        surf.merge(&tile);
    }

    for y in 0..surf.height {
        for x in 0..surf.width {
            let color = surf[(x, y)];
            if color.r != 255 {
                panic!("wrong pixel at {}x{}", x, y);
            }
            if color.g != 0 {
                panic!("wrong pixel at {}x{}", x, y);
            }
            if color.b != 0 {
                panic!("wrong pixel at {}x{}", x, y);
            }
        }
    }

    // Check the iterator too
    for color in surf.buffer.iter() {
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
    }
}
