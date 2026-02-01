use crate::{math::Vector3, types::Face};
use anyhow::Result;

use std::{
    fs::{self},
    path::Path,
};

#[derive(Default)]
pub struct ObjFile {
    pub verticies: Vec<Vector3<f64>>,
    pub faces: Vec<Face>,
}

fn parse_vertex(str: &str) -> Option<Vector3<f64>> {
    let mut itr = str.split(' ');
    let _ = itr.next();
    let x_opt = itr.next();
    let y_opt = itr.next();
    let z_opt = itr.next();
    if let (Some(x_str), Some(y_str), Some(z_str)) = (x_opt, y_opt, z_opt)
        && let (Ok(x), Ok(y), Ok(z)) = (
            x_str.parse::<f64>(),
            y_str.parse::<f64>(),
            z_str.parse::<f64>(),
        )
    {
        return Some(Vector3::new([x, y, z]));
    }
    None
}

fn parse_face(str: &str) -> Option<Face> {
    let mut itr = str.split(' ');
    itr.next();
    if let (Some(one_chunk_str), Some(two_chunk_str), Some(three_chunk_str)) =
        (itr.next(), itr.next(), itr.next())
        && let (Some((one_str, _)), Some((two_str, _)), Some((three_str, _))) = (
            one_chunk_str.split_once('/'),
            two_chunk_str.split_once('/'),
            three_chunk_str.split_once('/'),
        )
        && let (Ok(one), Ok(two), Ok(three)) = (
            one_str.parse::<usize>(),
            two_str.parse::<usize>(),
            three_str.parse::<usize>(),
        )
    {
        return Some(Face { one, two, three });
    }
    None
}

pub fn parse_obj_file(path: &Path) -> Result<ObjFile> {
    let file_string = fs::read_to_string(path)?;
    let mut verticies: Vec<Vector3<f64>> = Vec::new();
    let mut faces: Vec<Face> = Vec::new();
    for line in file_string.lines() {
        let mut elements_itr = line.split(' ');
        if let Some(first_element) = elements_itr.next() {
            match first_element {
                "v" => {
                    if let Some(vertex) = parse_vertex(line) {
                        verticies.push(vertex);
                    }
                }
                "f" => {
                    if let Some(face) = parse_face(line) {
                        faces.push(face);
                    }
                }
                _ => {}
            }
        }
    }
    Ok(ObjFile { verticies, faces })
}
