use std::f64::consts::PI;

use crate::{
    colors::Color,
    math::{Matrix, Vector3},
    obj::ObjFile,
    tga::{ColorSpace, Grayscale, Image, RGBA},
    triangle::Triangle,
    types::Point,
};
use anyhow::Result;

fn rotate(vec: &Vector3<f64>) -> Vector3<f64> {
    let a = PI / 6.;
    let cos_a = a.cos();
    let sin_a = a.sin();
    let r_y_vec_one = Vector3::new([cos_a, 0., sin_a]);
    let r_y_vec_two = Vector3::new([0., 1., 0.]);
    let r_y_vec_three = Vector3::new([-sin_a, 0., cos_a]);
    let r_y = Matrix::new([r_y_vec_one, r_y_vec_two, r_y_vec_three]);
    let temp = Matrix::new([vec.clone()]).transpose();
    (r_y * temp).to_vector()
}

fn project(vec: &Vector3<f64>, width: f64, height: f64) -> Vector3<isize> {
    Vector3::new([
        ((vec.x() + 1.0) * 0.5 * width).min(width - 1.0) as isize,
        ((vec.y() + 1.0) * 0.5 * height).min(width - 1.0) as isize,
        ((vec.z() + 1.0) * (255. / 2.)) as isize,
    ])
}

pub fn draw_obj_file<T: ColorSpace + Copy>(obj: ObjFile, img: &mut Image<T>) -> Result<()> {
    let width = img.width;
    let height = img.height;
    let verticies = obj.verticies;
    let faces = obj.faces;
    let mut z_buff = Image::<Grayscale>::new(width, height);
    let width_f64 = width as f64;
    let height_f64 = height as f64;

    for face in faces {
        if let (Some(vertex_one), Some(vertex_two), Some(vertex_three)) = (
            verticies.get(face.one - 1),
            verticies.get(face.two - 1),
            verticies.get(face.three - 1),
        ) {
            let triangle = Triangle {
                vector_a: project(&rotate(vertex_one), width_f64, height_f64),
                vector_b: project(&rotate(vertex_two), width_f64, height_f64),
                vector_c: project(&rotate(vertex_three), width_f64, height_f64),
            };

            // z index hack
            if triangle.area() > 1.0 {
                triangle.draw::<T>(T::random(), img, Some(&mut z_buff))?;
            }
        }
    }
    z_buff.write_to_file("z_buffer.tga", true, false)?;
    Ok(())
}

fn draw_line(
    point_one: &Point,
    point_two: &Point,
    img: &mut Image<RGBA>,
    color: &Color,
) -> Result<()> {
    let delta_x = point_one.x.abs_diff(point_two.x);
    let delta_y = point_one.y.abs_diff(point_two.y);
    let steep = delta_y > delta_x;
    let min_x: isize;
    let max_x: isize;
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
