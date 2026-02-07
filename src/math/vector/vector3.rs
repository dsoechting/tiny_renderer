use num::Num;

use crate::math::Vector;
use std::{fmt::Display, iter::Sum};

pub type Vector3<T> = Vector<T, 3>;

impl<T> Vector3<T>
where
    T: Num + Sum + Copy + Display,
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
}
