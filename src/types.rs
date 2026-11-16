use crate::math::Vector3;

#[derive(Debug, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
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

#[derive(Default)]
pub struct Mesh {
    pub verticies: Vec<Vector3<f64>>,
    pub faces: Vec<Face>,
}
