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
}
