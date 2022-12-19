pub mod grid {
    use std::fmt::Display;

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

    impl<T: Display + Copy, const SIZE: usize> Grid<T, SIZE> {
        pub fn to_str(&self) -> String {
            let mut result = String::new();

            for row in self.internal_grid {
                for cell in row {
                    result.push_str(&cell.to_string());
                }
                result.push('\n');
            }
            result
        }
    }
}
