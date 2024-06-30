use std::{
    ops::Index,
};

#[derive(Debug)]
struct Matrix<T> {
    num_rows: i32,
    num_cols: i32,
    data: Vec<T>,
}

#[derive(Debug)]
struct Vector<T> {
    data: Vec<T>,
}

impl<T> Vector<T> {
    fn new(data: Vec<T>) -> Self {
        Vector { data }
    }
}

impl<T: Clone> Index<usize> for Matrix<T> {
    type Output = Vector<T>;

    fn index(&self, i: usize) -> &Vector<T> {
        let index = i as i32;
        let start = (index * &self.num_cols) as usize;
        let end = ((index + 1) * &self.num_cols) as usize;
        let data_slice = &self.data[start..end];
        let data = data_slice.to_vec();
        let vector_temp = Vector::<T>::new(data);
        return &vector_temp;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index() {
        let matrix = Matrix {
            num_rows: 2,
            num_cols: 3,
            data: vec![1, 2, 3, 4, 5, 6],
        };
        let t = &matrix[0];
        assert!(format!("{:?}", t).contains("[1, 2, 3]"));
        let t = &matrix[1];
        assert!(format!("{:?}", t).contains("[4, 5, 6]"));
    }
}
