#[derive(Debug, Clone)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug, Clone)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

// Indexes of verticies
// Hard coding three ints. Will likely change down the line
// Just need to parse an obj file for now
#[derive(Debug)]
pub struct Face {
    pub one: usize,
    pub two: usize,
    pub three: usize,
}
