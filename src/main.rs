use std::{path::Path, time::Instant};

use anyhow::Result;
use tiny_renderer::{
    colors::Color,
    draw::{draw_line, draw_obj_file},
    obj::{ObjFile, parse_obj_file},
    tga::Image,
    types::Point,
};

fn main() {
    // draw_triangle().unwrap();
    let start = Instant::now();
    let head_path = Path::new("./assets/head.obj");
    let body_path = Path::new("./assets/body.obj");
    let diablo_path = Path::new("./assets/diablo.obj");
    let bear_path = Path::new("./assets/bear.obj");
    let path = bear_path;

    if let Ok(diablo_model) = parse_obj_file(path) {
        draw_obj_file(diablo_model);
    }
    let end = Instant::now();
    println!("Duration: {:?}", end - start);
}

fn draw_triangle() -> Result<()> {
    let width: usize = 64;
    let height: usize = 64;

    let point1 = Point { x: 7, y: 3 };
    let point2 = Point { x: 12, y: 37 };
    let point3 = Point { x: 62, y: 53 };

    let mut img = Image::new(width, height);

    // Triangle
    draw_line(&point1, &point2, &mut img, Color::Blue)?;
    draw_line(&point3, &point2, &mut img, Color::Green)?;
    draw_line(&point3, &point1, &mut img, Color::Yellow)?;
    draw_line(&point1, &point3, &mut img, Color::Red)?;
    img.set(point1.x, point1.y, Color::White.rgba_value())?;
    img.set(point2.x, point2.y, Color::White.rgba_value())?;
    img.set(point3.x, point3.y, Color::White.rgba_value())?;
    img.write_to_file("output.tga", true, true)?;
    Ok(())
}
