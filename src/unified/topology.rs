pub trait Topology {
    fn buffer_size(image_size: (u32, u32)) -> usize;

    fn get_offset(image_size: (u32, u32), position: (u32, u32)) -> usize;

    fn get_position(image_size: (u32, u32), offset: usize) -> (u32, u32);
}

// 128x8 tiles
pub struct Zig;

const ZIG_TILE_WIDTH: u32 = 128;
const ZIG_TILE_HEIGHT: u32 = 8;

impl Topology for Zig {
    fn buffer_size(image_size: (u32, u32)) -> usize {
        let (mut width, mut height) = image_size;
        if width % ZIG_TILE_WIDTH > 0 {
            width = (1 + width / ZIG_TILE_WIDTH) * ZIG_TILE_WIDTH;
        }
        if height % ZIG_TILE_HEIGHT > 0 {
            height = (1 + height / ZIG_TILE_HEIGHT) * ZIG_TILE_HEIGHT;
        }
        width as usize * height as usize
    }

    fn get_offset(image_size: (u32, u32), (x, y): (u32, u32)) -> usize {
        const TILE_SIZE: usize = ZIG_TILE_WIDTH as usize * ZIG_TILE_HEIGHT as usize;

        let (width, _height) = image_size;
        
        let tiles_across = width / ZIG_TILE_WIDTH;
        let tile_x = x / ZIG_TILE_WIDTH;
        let inner_x = x % ZIG_TILE_WIDTH;
        let tile_y = y / ZIG_TILE_HEIGHT;
        let inner_y = y % ZIG_TILE_HEIGHT;

        let mut idx: usize = 0;
        idx += tile_y as usize * tiles_across as usize + tile_x as usize;
        idx *= TILE_SIZE;
        idx += inner_y as usize * ZIG_TILE_WIDTH as usize + inner_x as usize;
        idx
    }

    fn get_position(image_size: (u32, u32), offset: usize) -> (u32, u32) {
        const TILE_SIZE: usize = ZIG_TILE_WIDTH as usize * ZIG_TILE_HEIGHT as usize;

        let (width, height) = image_size;
        
        let tile_num = offset / TILE_SIZE;
        let tile_offset = offset % TILE_SIZE;
        let tiles_across = width / ZIG_TILE_WIDTH;
        

        let maj_x = (tile_num % tiles_across as usize) as u32;
        let maj_y = (tile_num / tiles_across as usize) as u32;
        let (min_x, min_y) = Lines::get_position((ZIG_TILE_WIDTH, ZIG_TILE_HEIGHT), tile_offset);

        (maj_x * ZIG_TILE_WIDTH + min_x, maj_y * ZIG_TILE_HEIGHT + min_y)
    }
}

pub struct Lines;

impl Topology for Lines {
    fn buffer_size(image_size: (u32, u32)) -> usize {
        let (width, height) = image_size;
        width as usize * height as usize
    }

    fn get_offset(image_size: (u32, u32), (x, y): (u32, u32)) -> usize {
        let (width, _height) = image_size;
        x as usize + y as usize * width as usize
    }

    fn get_position(image_size: (u32, u32), offset: usize) -> (u32, u32) {
        let (width, _height) = image_size;
        (
            (offset % width as usize) as u32,
            (offset / width as usize) as u32,
        )
    }
}
