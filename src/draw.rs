use core::f64;
use std::ops::Mul;

use crate::{
    math::{Matrix, Vector, Vector3},
    obj::ObjFile,
    tga::{ColorSpace, Grayscale, Image},
    triangle::Triangle,
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

pub fn draw_obj_file<T: ColorSpace + Copy>(obj: ObjFile, img: &mut Image<T>) -> Result<()> {
    let width = img.width;
    let height = img.height;
    let f_width = width as f64;
    let f_height = height as f64;
    let verticies = obj.verticies;
    let faces = obj.faces;
    let mut z_buff = vec![vec![-1.; width]; height];

    // Could maybe move to main
    let eye = Vector3::new([-1, 0, 2]);
    let center = Vector3::new([0, 0, 0]);
    let up = Vector3::new([0, 1, 0]);
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
            let vertex_one_matrix = Matrix::new([Vector::new([
                vertex_one.x(),
                vertex_one.y(),
                vertex_one.z(),
                1.0,
            ])])
            .transpose();

            let vertex_two_matrix = Matrix::new([Vector::new([
                vertex_two.x(),
                vertex_two.y(),
                vertex_two.z(),
                1.0,
            ])])
            .transpose();

            let vertex_three_matrix = Matrix::new([Vector::new([
                vertex_three.x(),
                vertex_three.y(),
                vertex_three.z(),
                1.0,
            ])])
            .transpose();

            let vertex_one_composed_vector = (&composed_transformation * &vertex_one_matrix)
                .to_vector()
                .perspective_divide();
            // Need to convert vertex_one_composed_vector to matrix or impl vec * matrix when it
            // makes sense.
            // Need to apply viewport transform to our f64 values
            let vertex_two_composed_vector = (&composed_transformation * &vertex_two_matrix)
                .to_vector()
                .perspective_divide();
            let vertex_three_composed_vector = (&composed_transformation * &vertex_three_matrix)
                .to_vector()
                .perspective_divide();

            let viewport_vec_one = viewport_matrix
                .mul(&vertex_one_composed_vector.to_matrix())
                .to_vector()
                .drop_w();

            let viewport_vec_two = viewport_matrix
                .mul(&vertex_two_composed_vector.to_matrix())
                .to_vector()
                .drop_w();

            let viewport_vec_three = viewport_matrix
                .mul(&vertex_three_composed_vector.to_matrix())
                .to_vector()
                .drop_w();

            // We lose the Z value here
            let triangle = Triangle {
                vector_a: viewport_vec_one,
                vector_b: viewport_vec_two,
                vector_c: viewport_vec_three,
            };

            // z index hack
            if triangle.area() > 1.0 {
                triangle.draw::<T>(T::random(), img, &mut z_buff)?;
            }
        }
    }

    // #[cfg(debug_assertions)]
    // draw_z_buffer(&z_buff, width, height);

    Ok(())
}

#[cfg(debug_assertions)]
fn draw_z_buffer(z_buff: &[Vec<f64>], width: usize, height: usize) {
    let mut z_buff_img = Image::<Grayscale>::new(width, height);
    for (i, row) in z_buff.iter().enumerate() {
        for (j, z_value_f64) in row.iter().enumerate() {
            // This might break one day, idk. Seems like an aggressive cast
            let depth_color = Grayscale {
                i: *z_value_f64 as u8,
            };
            let _ = z_buff_img.set_pixel(i, j, depth_color);
        }
    }

    let _ = z_buff_img.write_to_file("z_buffer.tga", true, false);
}

pub fn draw_line(
    point_one: &Point,
    point_two: &Point,
    img: &mut Image<RGBA>,
    color: &Color,
) -> Result<()> {
    let delta_x = point_one.x.abs_diff(point_two.x);
    let delta_y = point_one.y.abs_diff(point_two.y);
    let steep = delta_y > delta_x;
    let min_x: i32;
    let max_x: i32;
    let point_one_x: f64;
    let point_one_y: f64;
    let point_two_x: f64;
    let point_two_y: f64;
    // If delta_y is greater than delta_x, flipping X and Y
    // Then we transpose it back at the very end.
    if steep {
        min_x = point_one.y.min(point_two.y);
        max_x = point_one.y.max(point_two.y);
        point_one_x = point_one.y as f64;
        point_one_y = point_one.x as f64;
        point_two_x = point_two.y as f64;
        point_two_y = point_two.x as f64;
    } else {
        min_x = point_one.x.min(point_two.x);
        max_x = point_one.x.max(point_two.x);
        point_one_x = point_one.x as f64;
        point_one_y = point_one.y as f64;
        point_two_x = point_two.x as f64;
        point_two_y = point_two.y as f64;
    }

    for x in min_x..=max_x {
        let t = (x as f64 - point_one_x) / (point_two_x - point_one_x);
        let line_y = (point_one_y + t * (point_two_y - point_one_y)) as usize;
        let x_usize = usize::try_from(x)?;
        if steep {
            img.set_pixel(line_y, x_usize, color.rgba_value())?;
        } else {
            img.set_pixel(x_usize, line_y, color.rgba_value())?;
        }
    }
    Ok(())
}
