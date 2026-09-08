use core::f64;
use std::f64::consts::PI;

use crate::{
    colors::Color,
    math::{Matrix, Vector, Vector3},
    obj::ObjFile,
    tga::{ColorSpace, Grayscale, Image, RGBA},
    triangle::Triangle,
    types::Point,
};
use anyhow::Result;

fn rotate(vec: &Vector3<f64>) -> Vector3<f64> {
    // Rotation factor. Likely want a param in the future
    // let a: f64 = PI;
    let a: f64 = 0.0;
    let cos_a = a.cos();
    let sin_a = a.sin();
    let r_y_vec_one = Vector3::new([cos_a, 0., sin_a]);
    let r_y_vec_two = Vector3::new([0., 1., 0.]);
    let r_y_vec_three = Vector3::new([-sin_a, 0., cos_a]);
    let r_y = Matrix::new([r_y_vec_one, r_y_vec_two, r_y_vec_three]);
    // TODO: Can I avoid the clone?
    let input_as_matrix = Matrix::new([vec.clone()]).transpose();
    (r_y * input_as_matrix).to_vector()
}

fn project(vec: Vector3<f64>, width: usize, height: usize) -> Vector3<i32> {
    let f_width = width as f64;
    let f_height = height as f64;

    let val1 = ((vec.x() + 1.0) * 0.5 * f_width) as i32;
    let val2 = ((vec.y() + 1.0) * 0.5 * f_height) as i32;
    let val3 = ((vec.z() + 1.0) * (255. / 2.)) as i32;

    Vector3::new([
        val1.min(width as i32 - 1),
        val2.min(height as i32 - 1),
        val3,
    ])
}

fn viewport(vec: Vector3<f64>, width: f64, height: f64) -> Matrix<f64, 4, 4> {
    let vec1 = Vector::new([(width / 2.), 0.0, 0.0, (vec.x() + (width / 2.))]);
    let vec2 = Vector::new([0.0, (height / 2.), 0.0, (vec.y() + (height / 2.))]);

    let vec3 = Vector::new([0.0, 0.0, 1.0, 0.0]);
    let vec4 = Vector::new([0.0, 0.0, 0.0, 1.0]);

    Matrix::new([vec1, vec2, vec3, vec4])
}

// This is the perspective matrix for homogenous coords
fn perspective2(focal: f64) -> Matrix<f64, 4, 4> {
    let vec1 = Vector::new([1.0, 0.0, 0.0, 0.0]);
    let vec2 = Vector::new([0.0, 1.0, 0.0, 0.0]);
    let vec3 = Vector::new([0.0, 0.0, 1.0, 0.0]);
    let vec4 = Vector::new([0.0, 0.0, -(1. / focal), 1.0]);
    Matrix::new([vec1, vec2, vec3, vec4])
}

fn look_at(eye: &Vector3<i32>, center: &Vector3<i32>, up: &Vector3<i32>) -> Matrix<f64, 4, 4> {
    let center_float = center.to_float();
    let n = (eye - center).normalize();
    let l = up.to_float().cross(&n).normalize();
    let m = n.cross(&l).normalize();
    let transformation_matrix = Matrix::new([
        Vector::new([l.x(), l.y(), l.z(), 0.0]),
        Vector::new([m.x(), m.y(), m.z(), 0.0]),
        Vector::new([n.x(), n.y(), n.z(), 0.0]),
        Vector::new([0.0, 0.0, 0.0, 1.0]),
    ]);
    let translation_matrix = Matrix::new([
        Vector::new([1.0, 0.0, 0.0, -center_float.x()]),
        Vector::new([0.0, 1.0, 0.0, -center_float.y()]),
        Vector::new([0.0, 0.0, 1.0, -center_float.z()]),
        Vector::new([0.0, 0.0, 0.0, 1.0]),
    ]);

    transformation_matrix * translation_matrix
}

fn perspective(vec: Vector3<f64>) -> Vector3<f64> {
    let camera = 3.;
    let scalar = 1. - vec.z() / camera;
    vec / scalar
}

pub fn draw_obj_file<T: ColorSpace + Copy>(obj: ObjFile, img: &mut Image<T>) -> Result<()> {
    let width = img.width;
    let height = img.height;
    let verticies = obj.verticies;
    let faces = obj.faces;
    let mut z_buff = vec![vec![0.; width]; height];

    for face in faces {
        if let (Some(vertex_one), Some(vertex_two), Some(vertex_three)) = (
            verticies.get(face.one - 1),
            verticies.get(face.two - 1),
            verticies.get(face.three - 1),
        ) {
            let triangle = Triangle {
                vector_a: project(perspective(rotate(vertex_one)), width, height),
                vector_b: project(perspective(rotate(vertex_two)), width, height),
                vector_c: project(perspective(rotate(vertex_three)), width, height),
            };

            // z index hack
            if triangle.area() > 1.0 {
                triangle.draw::<T>(T::random(), img, Some(&mut z_buff))?;
            }
        }
    }

    #[cfg(debug_assertions)]
    draw_z_buffer(&z_buff, width, height);

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
