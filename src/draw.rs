use crate::{
    colors::Color,
    obj::ObjFile,
    tga::{ColorSpace, Image, RGBA},
    triangle::Triangle,
    types::Point,
};
use anyhow::Result;

pub fn draw_obj_file(obj: ObjFile, img: &mut Image<RGBA>) -> Result<()> {
    let width = img.width;
    let height = img.height;
    let verticies = obj.verticies;
    let faces = obj.faces;
    for face in faces {
        if let (Some(vertex_one), Some(vertex_two), Some(vertex_three)) = (
            verticies.get(face.one - 1),
            verticies.get(face.two - 1),
            verticies.get(face.three - 1),
        ) {
            let point_a = Point {
                x: (((vertex_one.x + 1.0) * 0.5 * width as f64) as isize).min((width - 1) as isize),
                y: (((vertex_one.y + 1.0) * 0.5 * height as f64) as isize)
                    .min((width - 1) as isize),
            };

            let point_b = Point {
                x: (((vertex_two.x + 1.0) * 0.5 * width as f64) as isize).min((width - 1) as isize),
                y: (((vertex_two.y + 1.0) * 0.5 * height as f64) as isize)
                    .min((width - 1) as isize),
            };

            let point_c = Point {
                x: (((vertex_three.x + 1.0) * 0.5 * width as f64) as isize)
                    .min((width - 1) as isize),
                y: (((vertex_three.y + 1.0) * 0.5 * height as f64) as isize)
                    .min((width - 1) as isize),
            };
            let triangle = Triangle {
                point_a,
                point_b,
                point_c,
            };

            // z index hack
            if triangle.area() > 1.0 {
                triangle.draw::<RGBA>(Color::Red.rgba_value(), img)?;
            }
        }
    }
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
