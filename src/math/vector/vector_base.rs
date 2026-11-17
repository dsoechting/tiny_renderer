use std::{
    array,
    fmt::{Debug, Display},
    iter::Sum,
    ops::{Add, Div, Index, IndexMut, Mul, Sub},
};

use num::{Num, ToPrimitive};

use crate::math::Vector3;

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
        let w = raw_data[3];
        let data: [f64; 4] = array::from_fn(|i| raw_data[i].div(w));
        Vector::new(data)
    }

    pub fn to_cartesian(&self) -> Vector3<f64> {
        let raw_data = self.data;
        let data: [f64; 3] = array::from_fn(|i| raw_data[i]);
        Vector3::new(data)
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
            data: self.data.map(|element| element / rhs),
        }
    }
}

impl<T, const N: usize> Vector<T, N>
where
    T: Display + Sum + Copy + Into<f64> + Num,
{
    pub fn magnitude(&self) -> f64 {
        let dot_with_self: f64 = self.dot(self).into();
        dot_with_self.sqrt()
    }

    pub fn normalize(&self) -> Vector<f64, N> {
        let magnitude = self.magnitude();
        let arr: [f64; N] = array::from_fn(|i| {
            let curr_data: f64 = self.data[i].into();
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
    use num::Float;

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
