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
    type Output = Vector<T>;

    fn index(&self, i: usize) -> &Vector<T> {
        let index = i as i32;
        let start = (index * &self.num_cols) as usize;
        let end = (((index + 1) * &self.num_cols) - 1) as usize;
        let data_slice = &self.data[start..end];
        let data = data_slice.to_vec();
        let vector_temp = Vector::<T>::new(data);
        return &vector_temp;
    }
}

fn main() {}