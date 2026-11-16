use num::Num;

use crate::math::Vector;
use std::{fmt::Display, iter::Sum};

pub type Vector3<T> = Vector<T, 3>;

impl<T> Vector3<T>
where
    T: Num + Sum + Copy + Display + Into<f64>,
{
    pub fn x(&self) -> T {
        self.get_data()[0]
    }

    pub fn y(&self) -> T {
        self.get_data()[1]
    }

    pub fn z(&self) -> T {
        self.get_data()[2]
    }

    pub fn cross(&self, other: &Self) -> Self {
        Vector::new([
            (self.y() * other.z()) - (self.z() * other.y()),
            (self.z() * other.x()) - (self.x() * other.z()),
            (self.x() * other.y()) - (self.y() * other.x()),
        ])
    }
}

#[cfg(test)]
mod test {
    use crate::math::Vector;

    #[test]
    fn vec3_cross_int() {
        let vec_one = Vector::new([1, 2, 3]);
        let vec_two = Vector::new([4, 5, 6]);

        let expected_result = Vector::new([-3, 6, -3]);

        let actual_result = vec_one.cross(&vec_two);
        assert_eq!(expected_result, actual_result)
    }
}
