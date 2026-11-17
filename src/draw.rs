use core::f64;
use std::ops::Mul;

use crate::utils::write_to_buffers::write_to_buffers;
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
    let mut z_buff = vec![vec![f64::MIN; width]; height];

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
                .to_cartesian();
            let vertex_two_transposed = viewport_matrix
                .mul(&(&composed_transformation * &vertex_two_homogenous).perspective_divide())
                .to_cartesian();
            let vertex_three_transposed = viewport_matrix
                .mul(&(&composed_transformation * &vertex_three_homogenous).perspective_divide())
                .to_cartesian();

            // We lose the Z value here
            let triangle = Triangle {
                vector_a: vertex_one_transposed,
                vector_b: vertex_two_transposed,
                vector_c: vertex_three_transposed,
            };

            // z index hack
            if triangle.area() > 1.0 {
                rasterize_triangle(&triangle, T::random(), img, &mut z_buff)?;
            }
        }
    }

    #[cfg(debug_assertions)]
    draw_z_buffer(&z_buff, width, height);

    Ok(())
}

pub fn rasterize_triangle<T: ColorSpace + Copy>(
    triangle: &Triangle,
    color: T,
    img: &mut Image<T>,
    z_buffer: &mut [Vec<f64>],
) -> Result<()> {
    let bb_min_x = triangle
        .vector_a
        .x()
        .min(triangle.vector_b.x())
        .min(triangle.vector_c.x())
        .floor() as i32;
    let bb_max_x = triangle
        .vector_a
        .x()
        .max(triangle.vector_b.x())
        .max(triangle.vector_c.x())
        .max(img.width as f64)
        .ceil() as i32;
    let bb_min_y = triangle
        .vector_a
        .y()
        .min(triangle.vector_b.y())
        .min(triangle.vector_c.y())
        .floor() as i32;
    let bb_max_y = triangle
        .vector_a
        .y()
        .max(triangle.vector_b.y())
        .max(triangle.vector_c.y())
        .max(img.height as f64)
        .ceil() as i32;

    let total_area = triangle.area();
    // Figure out how to do this in parallel
    // Use rayon to give individual rows of frame buffer
    for y in bb_min_y..=bb_max_y {
        for x in bb_min_x..=bb_max_x {
            let current_point = Vector3::new([x as f64, y as f64, 0.0]);
            let alpha: f64 =
                Triangle::create_from_refs(&current_point, &triangle.vector_b, &triangle.vector_c)
                    .area()
                    / total_area;
            let beta: f64 =
                Triangle::create_from_refs(&current_point, &triangle.vector_c, &triangle.vector_a)
                    .area()
                    / total_area;
            let gamma: f64 =
                Triangle::create_from_refs(&current_point, &triangle.vector_a, &triangle.vector_b)
                    .area()
                    / total_area;
            if alpha.is_sign_negative() || beta.is_sign_negative() || gamma.is_sign_negative() {
                continue;
            }
            let z = alpha * triangle.vector_a.z()
                + beta * triangle.vector_b.z()
                + gamma * triangle.vector_c.z();

            // Filter out negative values, they're off screen
            if let (Ok(x_unsigned), Ok(y_unsigned)) = (usize::try_from(x), usize::try_from(y)) {
                write_to_buffers(z_buffer, img, x_unsigned, y_unsigned, z, color)?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{
        colors::Color,
        draw::rasterize_triangle,
        math::Vector3,
        tga::{Image, RGBA},
        triangle::Triangle,
    };

    #[test]
    fn test_triangles() {
        let width: usize = 128;
        let height: usize = 128;
        let mut img = Image::<RGBA>::new(width, height);
        let mut z_buff = vec![vec![f64::MIN; width]; height];

        // Trianlge 1
        let vector_a = Vector3::new([7.0, 45.0, 0.0]);
        let vector_b = Vector3::new([35.0, 100.0, 0.0]);
        let vector_c = Vector3::new([45.0, 60.0, 0.0]);
        let triangle_1 = Triangle {
            vector_a,
            vector_b,
            vector_c,
        };

        // Triangle 2
        let vector_d = Vector3::new([120.0, 35.0, 0.0]);
        let vector_e = Vector3::new([90.0, 5.0, 0.0]);
        let vector_f = Vector3::new([45.0, 110.0, 0.0]);
        let triangle_2 = Triangle {
            vector_a: vector_d,
            vector_b: vector_e,
            vector_c: vector_f,
        };

        // Triangle 3
        let vector_g = Vector3::new([115.0, 83.0, 0.0]);
        let vector_h = Vector3::new([80.0, 90.0, 0.0]);
        let vector_i = Vector3::new([85.0, 120.0, 0.0]);
        let triangle_3 = Triangle {
            vector_a: vector_g,
            vector_b: vector_h,
            vector_c: vector_i,
        };
        let _ = rasterize_triangle(&triangle_1, Color::Red.rgba_value(), &mut img, &mut z_buff);
        let _ = rasterize_triangle(
            &triangle_2,
            Color::White.rgba_value(),
            &mut img,
            &mut z_buff,
        );
        let _ = rasterize_triangle(
            &triangle_3,
            Color::Green.rgba_value(),
            &mut img,
            &mut z_buff,
        );

        let _ = img.write_to_file("triangles.tga", true, true);
    }
}
