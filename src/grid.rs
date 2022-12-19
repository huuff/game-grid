pub mod grid {
    pub struct Grid<T: Sized + Copy, const SIZE: usize> {
        internal_grid: [[T; SIZE]; SIZE],
    }

    impl<T: Sized + Copy, const SIZE: usize> Grid<T, SIZE> {
        pub fn new(initial_value: T) -> Grid<T, SIZE> {
            Grid {
              internal_grid: [[initial_value; SIZE]; SIZE]  
            }
        }

        pub fn get(&self, (x, y): (usize, usize)) -> Option<&T> {
            self.internal_grid.get(y)?.get(x)
        }

        pub fn set(&mut self, (x, y): (usize, usize), value: T) {
            self.internal_grid[y][x] = value;
        }
    }
}
