use std::{
    array::{self},
    fmt::Display,
    iter::Sum,
    ops::{Add, Mul, Sub},
};

use crate::math::Vector;

#[derive(Debug)]
pub struct Matrix<T, const R: usize, const C: usize>
where
    T: Add + Sub + Mul + PartialEq + Copy + Display + Sum,
{
    data: [Vector<T, C>; R],
}

impl<T, const R: usize, const C: usize> Matrix<T, R, C>
where
    T: Add<Output = T> + Sum + Sub + Mul<Output = T> + PartialEq + Copy + Display,
{
    pub fn new(data: [Vector<T, C>; R]) -> Matrix<T, R, C> {
        Matrix { data }
    }
    fn rows(&self) -> [Vector<T, C>; R] {
        self.data
    }
    fn columns(&self) -> [Vector<T, R>; C] {
        array::from_fn(|i| Vector::new(self.data.map(|row| row.get_data()[i])))
    }
}

impl<T, const R: usize, const C: usize> PartialEq for Matrix<T, R, C>
where
    T: Sum + Add + Sub + Mul + PartialEq + Copy + Display,
{
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

// Matrix Multiplication notes
// Row count 1 must match Column count 2
// If we have Matrix<R, C> * Matric<R', C'>, the final Matrix will be Matrix<R, C'>
// Multiply row vector (virtual) of matrix 1 by column vector of matrix 2
// Might be worth making a function to get a row vector, since we already store the matrix as
// column vectors.
//
//row1 * column1, row1 * column2, row2 * column1, row2 * column2
impl<T, const R: usize, const C: usize, const K: usize> Mul<Matrix<T, C, K>> for Matrix<T, R, C>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + PartialEq + Copy + Display + Sum,
{
    type Output = Matrix<T, R, K>;
    fn mul(self, other: Matrix<T, C, K>) -> Self::Output {
        let self_rows = self.rows();
        let other_cols = other.columns();

        let data = self_rows
            .map(|self_row| Vector::new(other_cols.map(|other_col| self_row.dot(other_col))));

        Matrix { data }
    }
}

#[cfg(test)]
mod test {
    use std::ops::Mul;

    use crate::math::Matrix;
    use crate::math::Vector;

    #[test]
    fn matrix_cols_test() {
        // let expected_vec = Vector::new([5, 7, 9]);
        let vec_expected_one = Vector::new([1, 4]);
        let vec_expected_two = Vector::new([2, 5]);
        let vec_expected_three = Vector::new([3, 6]);
        let expected = [vec_expected_one, vec_expected_two, vec_expected_three];

        let vec_one = Vector::new([1, 2, 3]);
        let vec_two = Vector::new([4, 5, 6]);
        let matrix = Matrix::new([vec_one, vec_two]);
        let result = matrix.columns();

        assert_eq!(expected, result);
    }

    #[test]
    fn matrix_rows_test() {
        let vec_expected_one = Vector::new([1, 2, 3]);
        let vec_expected_two = Vector::new([4, 5, 6]);
        let expected = [vec_expected_one, vec_expected_two];

        let vec_one = Vector::new([1, 2, 3]);
        let vec_two = Vector::new([4, 5, 6]);
        let matrix = Matrix::new([vec_one, vec_two]);
        let result = matrix.rows();

        assert_eq!(expected, result);
    }

    #[test]
    fn matrix_mult_test() {
        let vec_expected_one = Vector::new([21, 27]);
        let vec_expected_two = Vector::new([48, 63]);
        let expected = Matrix::new([vec_expected_one, vec_expected_two]);

        let vec_one = Vector::new([1, 2, 3]);
        let vec_two = Vector::new([4, 5, 6]);
        let matrix = Matrix::new([vec_one, vec_two]);

        let vec_three = Vector::new([1, 2]);
        let vec_four = Vector::new([4, 5]);
        let vec_five = Vector::new([4, 5]);
        let matrix_two = Matrix::new([vec_three, vec_four, vec_five]);

        let result = matrix.mul(matrix_two);

        assert_eq!(expected, result);
    }
}
