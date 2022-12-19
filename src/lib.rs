mod grid;

use grid::grid::Grid;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create() {
        let _grid: Grid<u8, 12> = Grid::new(0u8);
    }
}
