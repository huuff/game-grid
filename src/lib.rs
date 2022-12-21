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

    #[test]
    // TODO: There's no assert here
    fn prints_correctly() {
        let mut grid: Grid<char, 3> = Grid::new('O');

        grid.set((1,1), 'X');

        println!("\n{}\n", grid.to_str());
    }

    #[test]
    fn indexing_operator() {
        let mut grid: Grid<char, 3> = Grid::new('O');

        grid.set((1,1), 'X');
        assert_eq!(grid[(1,1)], 'X');
    }
}
