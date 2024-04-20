#[derive(Default)]
pub struct GameMap([u16; 15]);

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
