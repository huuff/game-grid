pub mod grid {
    use std::{fmt::Display, ops::Index};

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

    pub trait CharRepresentable {
        fn to_char(&self) -> char;
    }

    impl CharRepresentable for char {
        fn to_char(&self) -> char {
            self.clone()
        }
    }

    impl<T: CharRepresentable + Copy, const SIZE: usize> Grid<T, SIZE> {
        pub fn to_str(&self) -> String {
            let mut result = String::new();

            for row in self.internal_grid {
                for cell in row {
                    result.push(cell.to_char());
                }
                result.push('\n');
            }
            result
        }
    }

    impl<T: Sized + Copy, const SIZE: usize> Index<(usize, usize)> for Grid<T, SIZE> {
        type Output = T;

        fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
            &self.internal_grid[y][x]
        }
    }
}
