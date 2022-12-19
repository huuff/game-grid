mod grid;

use grid::grid::Grid;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_and_get() {
        let grid: Grid<u8, 12> = Grid::new(0u8);
        assert_eq!(*grid.get((2,2)).unwrap(), 0u8);
    }

    #[test]
    fn can_set_and_get() {
        let mut grid: Grid<u8, 12> = Grid::new(0u8);
        grid.set((2,2), 3u8);
        assert_eq!(*grid.get((2,2)).unwrap(), 3u8);
    }
}
