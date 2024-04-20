use bevy::math::UVec2;
use bevy_pixel_buffer::prelude::*;

pub const MAP_DIMS: PixelBufferSize = PixelBufferSize {
    size: UVec2::new(10, 20),
    pixel_size: UVec2::new(20, 20), 
};

#[derive(Default)]
pub struct GameMap([u16; 15]);

impl GameMap {
    fn flatten_pos(pos: UVec2) -> usize {
        ((pos.y * MAP_DIMS.size.x) + pos.x) as usize
    }
}

#[cfg(test)]
mod tests {
    use crate::GameMap;

    #[test]
    fn create_empty_map() {
        let map = GameMap::default();
        for num in map.0 {
            assert_eq!(num, 0);
        }
    }

    #[test]
    fn bit_shifts() {
        let num = 0b01100101;
        assert_eq!(num & 1, 1);
        assert_eq!(num >> 1 & 1, 0);
        assert_eq!(num >> 2 & 1, 1);
        assert_eq!(num >> 3 & 1, 0);
        assert_eq!(num >> 4 & 1, 0);
        assert_eq!(num >> 5 & 1, 1);
        assert_eq!(num >> 6 & 1, 1);
        assert_eq!(num >> 7 & 1, 0);
    }
}
