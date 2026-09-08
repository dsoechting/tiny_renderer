use std::{
    array,
    fmt::{Debug, Display},
    iter::Sum,
    ops::{Add, Div, Index, IndexMut, Mul, Sub},
};

use num::{Integer, Num};

#[derive(Debug, Clone)]
pub struct Vector<T, const N: usize>
where
    T: Num + Sum + Copy + Display + Into<f64>,
{
    data: [T; N],
}

// General impl block
impl<T, const N: usize> Vector<T, N>
where
    T: Num + Sum + Copy + Display + Into<f64>,
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

    // Not certain that this is the correct name for this function
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

// Add impl block
impl<T, const N: usize> Add for Vector<T, N>
where
    T: Num + Sum + Copy + Display + Into<f64>,
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
    T: Num + Sum + Copy + Display + Into<f64>,
{
    type Output = Vector<T, N>;
    fn sub(self, other: Self) -> Self::Output {
        let data = array::from_fn(|i| self[i] - other[i]);
        Vector::new(data)
    }
}

impl<T, const N: usize> Sub for Vector<T, N>
where
    T: Num + Sum + Copy + Display + Into<f64>,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        (&self).sub(&other)
    }
}

impl<T, const N: usize> Div<T> for &Vector<T, N>
where
    T: Num + Sum + Copy + Display + Debug + Into<f64>,
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
    T: Integer + Into<f64> + Display + Sum + Copy,
{
    pub fn to_float(&self) -> Vector<f64, N> {
        let arr: [f64; N] = array::from_fn(|i| {
            let curr_data: f64 = self.data[i].into();
            curr_data
        });
        Vector::new(arr)
    }
}

impl<T, const N: usize> Div<T> for Vector<T, N>
where
    T: Num + Sum + Copy + Display + Debug + Into<f64>,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        (&self).div(rhs)
    }
}

impl<T, const N: usize> Mul<T> for &Vector<T, N>
where
    T: Num + Sum + Copy + Display + Into<f64>,
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
    T: Num + Sum + Copy + Display + Into<f64>,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        (&self).mul(rhs)
    }
}

// Display impl block
impl<T, const N: usize> Display for Vector<T, N>
where
    T: Num + Sum + Copy + Display + Into<f64>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res: String = self.data.iter().map(|n| format!("{}, ", n)).collect();
        writeln!(f, "Vector: {}", res)
    }
}

// Num impl block
impl<T, const N: usize> PartialEq for Vector<T, N>
where
    T: Num + Sum + Copy + Display + Into<f64>,
{
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<T, const N: usize> Index<usize> for Vector<T, N>
where
    T: Num + Sum + Copy + Display + Into<f64>,
{
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T, const N: usize> IndexMut<usize> for Vector<T, N>
where
    T: Num + Sum + Copy + Display + Into<f64>,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

#[cfg(test)]
mod test {
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
}
