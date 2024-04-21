use bevy::math::UVec2;
use bevy_pixel_buffer::prelude::*;

pub const MAP_DIMS: PixelBufferSize = PixelBufferSize {
    size: UVec2::new(10, 20),
    pixel_size: UVec2::new(20, 20), 
};

#[derive(Default)]
pub struct GameMap([u16; 15]);

impl GameMap {
}

fn check_if_pos_inbounds(pos: &UVec2) {
    if
        MAP_DIMS.size.x < pos.x ||
        MAP_DIMS.size.y < pos.y
    {
        panic!("Trying to check for an out-of-bounds position!");
    }
}

fn flatten_pos(pos: &UVec2) -> usize {
    check_if_pos_inbounds(pos);
    ((pos.y * MAP_DIMS.size.x) + pos.x) as usize
}

#[cfg(test)]
mod tests {
    use bevy::math::UVec2;

    use crate::GameMap;
    use crate::flatten_pos;

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

    #[test]
    fn flatten_some_positions() {
        assert_eq!(0, flatten_pos(&UVec2::ZERO));
        assert_eq!(3, flatten_pos(&UVec2::new(3, 0)));
        assert_eq!(13, flatten_pos(&UVec2::new(3, 1)));
        assert_eq!(47, flatten_pos(&UVec2::new(7, 4)));
    }

    #[test]
    fn bit_manipulation() {
        let number = 0b1010_1010;
        let mask = 0b0011_0011;
        assert_eq!(number | mask, 0b1011_1011);
        assert_eq!(number & mask, 0b0010_0010);
        assert_eq!(number ^ mask, 0b1001_1001);
    }
}
