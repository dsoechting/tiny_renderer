use core::f64;
use std::ops::Mul;

#[cfg(debug_assertions)]
use crate::{
    math::{Matrix, Vector, Vector3},
    tga::{ColorSpace, Image},
    triangle::Triangle,
    types::Mesh,
    utils::draw_z_buffer::draw_z_buffer,
};
use anyhow::Result;

fn viewport(x: f64, y: f64, width: f64, height: f64) -> Matrix<f64, 4, 4> {
    let vec1 = Vector::new([(width / 2.), 0.0, 0.0, (x + (width / 2.))]);
    let vec2 = Vector::new([0.0, (height / 2.), 0.0, (y + (height / 2.))]);

    let vec3 = Vector::new([0.0, 0.0, 1.0, 0.0]);
    let vec4 = Vector::new([0.0, 0.0, 0.0, 1.0]);

    Matrix::new([vec1, vec2, vec3, vec4])
}

// This is the perspective matrix for homogenous coords
fn perspective(focal: f64) -> Matrix<f64, 4, 4> {
    let vec1 = Vector::new([1.0, 0.0, 0.0, 0.0]);
    let vec2 = Vector::new([0.0, 1.0, 0.0, 0.0]);
    let vec3 = Vector::new([0.0, 0.0, 1.0, 0.0]);
    let vec4 = Vector::new([0.0, 0.0, -(1. / focal), 1.0]);
    Matrix::new([vec1, vec2, vec3, vec4])
}

fn look_at(eye: &Vector3<f64>, center: &Vector3<f64>, up: &Vector3<f64>) -> Matrix<f64, 4, 4> {
    let n = (eye - center).normalize();
    let l = up.cross(&n).normalize();
    let m = n.cross(&l).normalize();
    let transformation_matrix = Matrix::new([
        Vector::new([l.x(), l.y(), l.z(), 0.0]),
        Vector::new([m.x(), m.y(), m.z(), 0.0]),
        Vector::new([n.x(), n.y(), n.z(), 0.0]),
        Vector::new([0.0, 0.0, 0.0, 1.0]),
    ]);
    let translation_matrix = Matrix::new([
        Vector::new([1.0, 0.0, 0.0, -center.x()]),
        Vector::new([0.0, 1.0, 0.0, -center.y()]),
        Vector::new([0.0, 0.0, 1.0, -center.z()]),
        Vector::new([0.0, 0.0, 0.0, 1.0]),
    ]);

    &transformation_matrix * &translation_matrix
}

pub fn draw_mesh<T: ColorSpace + Copy>(mesh: Mesh, img: &mut Image<T>) -> Result<()> {
    let width = img.width;
    let height = img.height;
    let f_width = width as f64;
    let f_height = height as f64;
    let verticies = mesh.verticies;
    let faces = mesh.faces;
    let mut z_buff = vec![vec![-1.; width]; height];

    // Could maybe move to main
    let eye = Vector3::new([-1.0, 0.0, 2.0]);
    let center = Vector3::new([0.0, 0.0, 0.0]);
    let up = Vector3::new([0.0, 1.0, 0.0]);
    let model_view_matrix = look_at(&eye, &center, &up);

    let perspective_matrix = perspective((eye - center).magnitude());

    let composed_transformation = &perspective_matrix * &model_view_matrix;
    let viewport_matrix = viewport(
        f_width / 16.0,
        f_height / 16.0,
        f_width * (7.0 / 8.0),
        f_height * (7.0 / 8.0),
    );

    for face in faces {
        if let (Some(vertex_one), Some(vertex_two), Some(vertex_three)) = (
            verticies.get(face.one - 1),
            verticies.get(face.two - 1),
            verticies.get(face.three - 1),
        ) {
            //Convert our vertex to a 4D point for homogenous coordinates
            let vertex_one_homogeneous =
                Vector::new([vertex_one.x(), vertex_one.y(), vertex_one.z(), 1.0]);

            let vertex_two_homogenous =
                Vector::new([vertex_two.x(), vertex_two.y(), vertex_two.z(), 1.0]);

            let vertex_three_homogenous =
                Vector::new([vertex_three.x(), vertex_three.y(), vertex_three.z(), 1.0]);

            let vertex_one_transposed = viewport_matrix
                .mul(&(&composed_transformation * &vertex_one_homogeneous).perspective_divide())
                .drop_w();
            let vertex_two_transposed = viewport_matrix
                .mul(&(&composed_transformation * &vertex_two_homogenous).perspective_divide())
                .drop_w();
            let vertex_three_transposed = viewport_matrix
                .mul(&(&composed_transformation * &vertex_three_homogenous).perspective_divide())
                .drop_w();

            // We lose the Z value here
            let triangle = Triangle {
                vector_a: vertex_one_transposed,
                vector_b: vertex_two_transposed,
                vector_c: vertex_three_transposed,
            };

            // z index hack
            if triangle.area() > 1.0 {
                triangle.draw::<T>(T::random(), img, &mut z_buff)?;
            }
        }
    }

    #[cfg(debug_assertions)]
    draw_z_buffer(&z_buff, width, height);

    Ok(())
}
