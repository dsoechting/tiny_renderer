use crate::{
    colors::Color,
    obj::ObjFile,
    tga::{Image, RGBA},
    types::Point,
};
use anyhow::Result;

pub fn draw_obj_file(obj: ObjFile) -> Result<()> {
    let width: usize = 1600;
    let height: usize = 1600;
    let mut img = Image::new(width, height);
    let verticies = obj.verticies;
    let faces = obj.faces;
    for face in faces {
        if let (Some(vertex_one), Some(vertex_two), Some(vertex_three)) = (
            verticies.get(face.one - 1),
            verticies.get(face.two - 1),
            verticies.get(face.three - 1),
        ) {
            // dbg!(&vertex_one, &vertex_two, &vertex_three);
            let point1 = Point {
                x: ((vertex_one.x + 1.0) * 0.5 * width as f64) as usize,
                y: ((vertex_one.y + 1.0) * 0.5 * height as f64) as usize,
            };

            let point2 = Point {
                x: ((vertex_two.x + 1.0) * 0.5 * width as f64) as usize,
                y: ((vertex_two.y + 1.0) * 0.5 * height as f64) as usize,
            };

            let point3 = Point {
                x: ((vertex_three.x + 1.0) * 0.5 * width as f64) as usize,
                y: ((vertex_three.y + 1.0) * 0.5 * height as f64) as usize,
            };
            // dbg!(&point1, &point2, &point3);
            draw_line(&point1, &point2, &mut img, Color::Red);
            draw_line(&point1, &point3, &mut img, Color::Red);
            draw_line(&point3, &point2, &mut img, Color::Red);
        }
    }
    img.write_to_file("model.tga", true, true)?;
    Ok(())
}

pub fn draw_line(
    point_one: &Point,
    point_two: &Point,
    img: &mut Image<RGBA>,
    color: Color,
) -> Result<()> {
    let delta_x = point_one.x.abs_diff(point_two.x);
    let delta_y = point_one.y.abs_diff(point_two.y);
    let steep = delta_y > delta_x;
    let min_x: usize;
    let max_x: usize;
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
        if steep {
            img.set(line_y, x, color.rgba_value())?;
        } else {
            img.set(x, line_y, color.rgba_value())?;
        }
    }
    img.set(point_one.x, point_one.y, Color::White.rgba_value())?;
    img.set(point_two.x, point_two.y, Color::White.rgba_value())?;
    Ok(())
}
