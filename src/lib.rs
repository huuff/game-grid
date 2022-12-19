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
}
