use std::{
    fmt::Display,
    iter::Sum,
    ops::{Add, Mul, Sub},
};

#[derive(Debug, Clone, Copy)]
pub struct Vector<T, const N: usize>
where
    T: Add + Sub + Mul + PartialEq + Copy + Display,
{
    data: [T; N],
}

impl<T, const N: usize> Vector<T, N>
where
    T: Add<Output = T> + Sum + Sub + Mul<Output = T> + PartialEq + Copy + Display,
{
    pub fn new(data: [T; N]) -> Self {
        Self { data }
    }
    pub fn dot(&self, other: Self) -> T {
        let res: T = self.data.iter().zip(other.data).map(|(&x, y)| x * y).sum();
        res
    }

    pub fn get_data(&self) -> [T; N] {
        self.data
    }

    pub fn get<const I: usize>(&self) -> &T {
        &self.data[I]
    }
}

impl<T, const N: usize> Add for Vector<T, N>
where
    T: Add<Output = T> + Sub + Mul + PartialEq + Copy + Display,
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

impl<T, const N: usize> Sub for Vector<T, N>
where
    T: Add + Sub<Output = T> + Mul + PartialEq + Copy + Display,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        let mut out = self;
        for i in 0..N {
            out.data[i] = out.data[i] - other.data[i];
        }
        out
    }
}

impl<T, const N: usize> Display for Vector<T, N>
where
    T: Add + Sub + Mul + PartialEq + Copy + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res: String = self.data.iter().map(|n| format!("{}, ", n)).collect();
        writeln!(f, "Vector: {}", res)
    }
}

impl<T, const N: usize> PartialEq for Vector<T, N>
where
    T: Add + Sub + Mul + PartialEq + Copy + Display,
{
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

#[cfg(test)]
mod test {
    use crate::math::vector::Vector;

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

        let res = vec_one.dot(vec_two);
        assert_eq!(res, 32);
    }

    #[test]
    fn vec3_dot_float() {
        let vec_one = Vector::new([1., 2., 3.]);
        let vec_two = Vector::new([4., 5., 6.]);

        let res = vec_one.dot(vec_two);
        assert_eq!(res, 32.);
    }
}
