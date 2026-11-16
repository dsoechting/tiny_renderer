use std::{
    array,
    fmt::{Debug, Display},
    iter::Sum,
    ops::{Add, Div, Index, IndexMut, Mul, Sub},
};

use num::{Num, ToPrimitive};

use crate::math::{Matrix, Vector3};

#[derive(Debug, Clone)]
pub struct Vector<T, const N: usize>
where
    T: Num + Sum + Copy + Display,
{
    data: [T; N],
}

// General impl block
impl<T, const N: usize> Vector<T, N>
where
    T: Num + Sum + Copy + Display,
{
    pub fn new(data: [T; N]) -> Self {
        Self { data }
    }
    pub fn dot(&self, other: &Self) -> T {
        let res: T = self.data.iter().zip(other.data).map(|(&x, y)| x * y).sum();
        res
    }

    pub fn get_data(&self) -> [T; N] {
        self.data
    }
}

impl Vector<f64, 4> {
    pub fn perspective_divide(&self) -> Vector<f64, 4> {
        let raw_data = self.data;
        let x = raw_data.first().unwrap();
        let y = raw_data.get(1).unwrap();
        let z = raw_data.get(2).unwrap();
        let w = raw_data.get(3).unwrap();
        Vector::new([x.div(*w), y.div(*w), z.div(*w), 1.0])
    }

    pub fn drop_w(&self) -> Vector3<f64> {
        let raw_data = self.data;
        let x = *raw_data.first().unwrap();
        let y = *raw_data.get(1).unwrap();
        let z = *raw_data.get(2).unwrap();
        Vector3::new([x, y, z])
    }

    pub fn to_matrix(&self) -> Matrix<f64, 4, 1> {
        let raw_data = self.data;
        let x = raw_data.first().unwrap();
        let y = raw_data.get(1).unwrap();
        let z = raw_data.get(2).unwrap();
        let w = raw_data.get(3).unwrap();
        Matrix::new([
            Vector::new([*x]),
            Vector::new([*y]),
            Vector::new([*z]),
            Vector::new([*w]),
        ])
    }
}

// Add impl block
impl<T, const N: usize> Add for Vector<T, N>
where
    T: Num + Sum + Copy + Display,
{
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        let mut out = self;
        for i in 0..N {
            out.data[i] = out.data[i] + other.data[i]
        }
        out
    }
}

// Sub impl block
impl<T, const N: usize> Sub for &Vector<T, N>
where
    T: Num + Sum + Copy + Display,
{
    type Output = Vector<T, N>;
    fn sub(self, other: Self) -> Self::Output {
        let data = array::from_fn(|i| self[i] - other[i]);
        Vector::new(data)
    }
}

impl<T, const N: usize> Sub for Vector<T, N>
where
    T: Num + Sum + Copy + Display,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        (&self).sub(&other)
    }
}

impl<T, const N: usize> Div<T> for &Vector<T, N>
where
    T: Num + Sum + Copy + Display + Debug,
{
    type Output = Vector<T, N>;
    fn div(self, rhs: T) -> Self::Output {
        Vector {
            data: self.data.map(|element| {
                // dbg!(element, rhs, element / rhs);
                element / rhs
            }),
        }
    }
}

impl<T, const N: usize> Vector<T, N>
where
    T: Display + Sum + Copy + ToPrimitive + Num,
{
    pub fn magnitude(&self) -> f64 {
        // TODO: fix unwrap
        let dot_with_self: f64 = self.dot(self).to_f64().unwrap();
        dot_with_self.sqrt()
    }

    pub fn normalize(&self) -> Vector<f64, N> {
        let magnitude = self.magnitude();
        let arr: [f64; N] = array::from_fn(|i| {
            // TODO: fix unwrap
            let curr_data: f64 = self.data[i].to_f64().unwrap();
            curr_data / magnitude
        });
        Vector::new(arr)
    }
}

impl<T, const N: usize> Div<T> for Vector<T, N>
where
    T: Num + Sum + Copy + Display + Debug,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        (&self).div(rhs)
    }
}

impl<T, const N: usize> Mul<T> for &Vector<T, N>
where
    T: Num + Sum + Copy + Display,
{
    type Output = Vector<T, N>;
    fn mul(self, rhs: T) -> Self::Output {
        Vector {
            data: self.data.map(|element| element * rhs),
        }
    }
}

impl<T, const N: usize> Mul<T> for Vector<T, N>
where
    T: Num + Sum + Copy + Display,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        (&self).mul(rhs)
    }
}

// Display impl block
impl<T, const N: usize> Display for &Vector<T, N>
where
    T: Num + Sum + Copy + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res: String = self.data.iter().map(|n| format!("{}, ", n)).collect();
        writeln!(f, "Vector: {}", res)
    }
}

// Num impl block
impl<T, const N: usize> PartialEq for Vector<T, N>
where
    T: Num + Sum + Copy + Display,
{
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<T, const N: usize> Index<usize> for Vector<T, N>
where
    T: Num + Sum + Copy + Display,
{
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T, const N: usize> IndexMut<usize> for Vector<T, N>
where
    T: Num + Sum + Copy + Display,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

#[cfg(test)]
mod test {
    use num::{Float, integer::Roots};

    use crate::math::Vector;

    #[test]
    fn vec3_add_int() {
        let expected_vec = Vector::new([5, 7, 9]);

        let vec_one = Vector::new([1, 2, 3]);
        let vec_two = Vector::new([4, 5, 6]);

        let res = vec_one + vec_two;

        assert_eq!(expected_vec, res);
    }

    #[test]
    fn vec3_add_float() {
        let expected_vec = Vector::new([5., 7., 9.]);

        let vec_one = Vector::new([1., 2., 3.]);
        let vec_two = Vector::new([4., 5., 6.]);

        let res = vec_one + vec_two;
        assert_eq!(expected_vec, res);
    }

    #[test]
    fn vec3_sub_int() {
        let expected_vec = Vector::new([-3, -3, -3]);

        let vec_one = Vector::new([1, 2, 3]);
        let vec_two = Vector::new([4, 5, 6]);

        let res = vec_one - vec_two;
        assert_eq!(expected_vec, res);
    }

    #[test]
    fn vec3_sub_float() {
        let expected_vec = Vector::new([-3., -3., -3.]);

        let vec_one = Vector::new([1., 2., 3.]);
        let vec_two = Vector::new([4., 5., 6.]);

        let res = vec_one - vec_two;
        assert_eq!(expected_vec, res);
    }

    #[test]
    fn vec3_dot_int() {
        let vec_one = Vector::new([1, 2, 3]);
        let vec_two = Vector::new([4, 5, 6]);

        let res = vec_one.dot(&vec_two);
        assert_eq!(res, 32);
    }

    #[test]
    fn vec3_dot_float() {
        let vec_one = Vector::new([1., 2., 3.]);
        let vec_two = Vector::new([4., 5., 6.]);

        let res = vec_one.dot(&vec_two);
        assert_eq!(res, 32.);
    }

    #[test]
    fn vec_3_int_magnitude() {
        let input = Vector::new([1, 2, 3]);

        let base: f64 = 14.0;
        let expected_result = base.sqrt();
        let actual_result = input.magnitude();

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn vec_3_int_normalize() {
        let input = Vector::new([1, 2, 3]);

        let denominator: f64 = 14.0.sqrt();
        let expected_result =
            Vector::new([1.0 / denominator, 2.0 / denominator, 3.0 / denominator]);
        let actual_result = input.normalize();

        assert_eq!(expected_result, actual_result);
    }
}
