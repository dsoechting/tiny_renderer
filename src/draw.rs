use crate::{
    colors::Color,
    obj::ObjFile,
    tga::{Image, RGBA},
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
            draw_triangle(&point_a, &point_b, &point_c, img, Color::Red)?;
        }
    }
    Ok(())
}

pub fn draw_triangle(
    point_a: &Point,
    point_b: &Point,
    point_c: &Point,
    img: &mut Image<RGBA>,
    color: Color,
) -> Result<()> {
    // Triangle
    let mut points = [point_a, point_b, point_c];
    points.sort_by_key(|p| p.y);
    let low_point = points.first().unwrap();
    let med_point = points.get(1).unwrap();
    let high_point = points.last().unwrap();
    let total_height = high_point.y - low_point.y;

    // Bottom half of triangle
    if low_point.y != med_point.y {
        let segment_height = med_point.y - low_point.y;
        for y in low_point.y..=med_point.y {
            let y_unsigned = usize::try_from(y)?;

            let x1_signed: isize =
                low_point.x + ((high_point.x - low_point.x) * (y - low_point.y)) / total_height;
            let x2_signed: isize =
                low_point.x + ((med_point.x - low_point.x) * (y - low_point.y)) / segment_height;
            let x_min_unsigned = usize::try_from(x1_signed.min(x2_signed))?;
            let x_max_unsigned = usize::try_from(x1_signed.max(x2_signed))?;

            for x in x_min_unsigned..=x_max_unsigned {
                img.set(x, y_unsigned, color.rgba_value())?;
            }
        }
    }

    if med_point.y != high_point.y {
        let segment_height = high_point.y - med_point.y;
        for y in med_point.y..=high_point.y {
            let y_unsigned = usize::try_from(y)?;

            let x1_signed: isize =
                low_point.x + ((high_point.x - low_point.x) * (y - low_point.y)) / total_height;
            let x2_signed =
                med_point.x + ((high_point.x - med_point.x) * (y - med_point.y)) / segment_height;

            let x_min_unsigned = usize::try_from(x1_signed.min(x2_signed))?;
            let x_max_unsigned = usize::try_from(x1_signed.max(x2_signed))?;

            for x in x_min_unsigned..=x_max_unsigned {
                img.set(x, y_unsigned, color.rgba_value())?;
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
            img.set(line_y, x_usize, color.rgba_value())?;
        } else {
            img.set(x_usize, line_y, color.rgba_value())?;
        }
    }
    Ok(())
}
