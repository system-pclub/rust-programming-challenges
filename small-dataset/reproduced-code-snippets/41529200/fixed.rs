use std::ops::Index;

struct Matrix<T> {
    num_rows: i32,
    num_cols: i32,
    data: Vec<T>
}

struct Vector<T> {
    data: Vec<T>
}

impl<T> Vector<T> {
    fn new(data: Vec<T>) -> Self {
        Vector { data }
    }
}

impl<T: Clone> Index<usize> for Matrix<T> {
    type Output = [T];

    fn index(&self, i: usize) -> &[T] {
        let index = i as i32;
        let start = (index * &self.num_cols) as usize;
        let end = (((index + 1) * &self.num_cols) - 1) as usize;
        &self.data[start..end]
    }
}

fn main() {}