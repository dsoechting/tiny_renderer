#[derive(Debug, Clone)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
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
