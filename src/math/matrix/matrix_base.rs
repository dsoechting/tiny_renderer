use std::{
    array::{self},
    fmt::{Debug, Display},
    iter::Sum,
    ops::Mul,
};

use num::{Float, Num};

use crate::math::Vector;

#[derive(Debug, Clone)]
pub struct Matrix<T, const R: usize, const C: usize>
where
    T: Num + Sum + Copy + Display,
{
    data: [Vector<T, C>; R],
}

// General impl block
impl<T, const R: usize, const C: usize> Matrix<T, R, C>
where
    T: Num + Sum + Copy + Display,
{
    pub fn new(data: [Vector<T, C>; R]) -> Matrix<T, R, C> {
        Matrix { data }
    }
    fn rows(&self) -> &[Vector<T, C>; R] {
        &self.data
    }
    fn rows_mutable(self) -> [Vector<T, C>; R] {
        self.data
    }

    fn columns(&self) -> [Vector<T, R>; C] {
        let rows = self.rows();
        array::from_fn(|i| {
            let vector_data = array::from_fn(|j| rows[j].get_data()[i]);
            Vector::new(vector_data)
        })
    }

    pub fn transpose(&self) -> Matrix<T, C, R> {
        Matrix::new(self.columns())
    }
}

// Impl block for inversion
// Only square matricies with floats can be inverted
// Maybe I can create another impl block
impl<T, const N: usize> Matrix<T, N, N>
where
    T: Float + Display + Sum + Debug,
{
    pub fn identity() -> Self {
        Self::new(array::from_fn(|i| {
            Vector::new(array::from_fn(
                |j| if i == j { T::one() } else { T::zero() },
            ))
        }))
    }
    pub fn invert(&self) -> Option<Self> {
        let left = self.clone();
        let right = Self::identity();
        let mut left_rows = left.rows_mutable();
        let mut right_rows = right.rows_mutable();

        for i in 0..N {
            if left_rows[i].get_data()[i] == T::zero() {
                let mut swap_row = None;

                let tmp = (i + 1)..N;
                for r in tmp {
                    if left_rows[r][i] != T::zero() {
                        swap_row = Some(r);
                        break;
                    }
                }

                if let Some(swap_row_index) = swap_row {
                    left_rows.swap(i, swap_row_index);
                    right_rows.swap(i, swap_row_index);
                }
            }

            let pivot = left_rows[i][i];
            if pivot == T::zero() {
                return None;
            }

            for j in 0..N {
                left_rows[i][j] = left_rows[i][j] / pivot;
                right_rows[i][j] = right_rows[i][j] / pivot;
            }

            for r in 0..N {
                if r == i {
                    continue;
                }

                let factor = left_rows[r][i];
                if factor == T::zero() {
                    continue;
                }

                for c in 0..N {
                    left_rows[r][c] = left_rows[r][c] - factor * left_rows[i][c];
                    right_rows[r][c] = right_rows[r][c] - factor * right_rows[i][c];
                }
            }
        }
        Some(Matrix::new(right_rows))
    }
}

// Impl PartialEq
impl<T, const R: usize, const C: usize> PartialEq for Matrix<T, R, C>
where
    T: Num + Sum + Copy + Display,
{
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

// Impl Mul
impl<T, const R: usize, const C: usize, const K: usize> Mul<Matrix<T, C, K>> for Matrix<T, R, C>
where
    T: Num + Sum + PartialEq + Display + Copy,
{
    type Output = Matrix<T, R, K>;
    fn mul(self, other: Matrix<T, C, K>) -> Self::Output {
        let self_rows = self.rows();
        let other_cols = other.columns();

        let data: [Vector<T, K>; R] =
            array::from_fn(|i| Vector::new(array::from_fn(|j| self_rows[i].dot(&other_cols[j]))));

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

        assert_eq!(expected, result.clone());
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

    #[test]
    fn matrix_transpose_test() {
        let vec_expected_one = Vector::new([1, 4]);
        let vec_expected_two = Vector::new([2, 5]);
        let vec_expected_three = Vector::new([3, 6]);
        let expected = Matrix::new([vec_expected_one, vec_expected_two, vec_expected_three]);

        let vec_one = Vector::new([1, 2, 3]);
        let vec_two = Vector::new([4, 5, 6]);
        let matrix = Matrix::new([vec_one, vec_two]);
        let result = matrix.transpose();

        assert_eq!(expected, result);
    }

    #[test]
    fn matrix_inverse_test() {
        let vec_expected_one = Vector::new([-2., 3.]);
        let vec_expected_two = Vector::new([3., -4.]);
        let expected = Matrix::new([vec_expected_one, vec_expected_two]);

        let vec_input_one = Vector::new([4., 3.]);
        let vec_input_two = Vector::new([3., 2.]);
        let input = Matrix::new([vec_input_one, vec_input_two]);

        let identity_vector_one = Vector::new([1., 0.]);
        let identity_vector_two = Vector::new([0., 1.]);
        let expected_identity = Matrix::new([identity_vector_one, identity_vector_two]);
        let identity: Matrix<f64, 2, 2> = Matrix::identity();
        assert_eq!(expected_identity, identity);

        let result = input.invert().unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn matrix_inverse_3x3_test() {
        let vec_expected_one = Vector::new([3., -1., 1.]);
        let vec_expected_two = Vector::new([-15., 6., -5.]);
        let vec_expected_three = Vector::new([5., -2., 2.]);
        let expected = Matrix::new([vec_expected_one, vec_expected_two, vec_expected_three]);

        let vec_input_one = Vector::new([2., 0., -1.]);
        let vec_input_two = Vector::new([5., 1., 0.]);
        let vec_input_three = Vector::new([0., 1., 3.]);
        let input = Matrix::new([vec_input_one, vec_input_two, vec_input_three]);

        let result = input.invert().unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn matrix_inverse_pivot_swap_test() {
        let vec_expected_one = Vector::new([-15. / 16., 1. / 4., 3. / 16.]);
        let vec_expected_two = Vector::new([3. / 8., -1. / 2., 1. / 8.]);
        let vec_expected_three = Vector::new([5. / 16., 1. / 4., -1. / 16.]);
        let expected = Matrix::new([vec_expected_one, vec_expected_two, vec_expected_three]);

        let vec_input_one = Vector::new([0., 1., 2.]);
        let vec_input_two = Vector::new([1., 0., 3.]);
        let vec_input_three = Vector::new([4., 5., 6.]);
        let input = Matrix::new([vec_input_one, vec_input_two, vec_input_three]);

        let result = input.invert().unwrap();

        assert_eq!(expected, result);
    }
}
