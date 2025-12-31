use std::{path::Path, time::Instant};

use anyhow::Result;
use tiny_renderer::{
    colors::Color,
    draw::{draw_obj_file, draw_triangle},
    obj::{ObjFile, parse_obj_file},
    tga::Image,
    types::Point,
};

fn main() {
    let start = Instant::now();
    test_obj_files();
    test_triangles();
    let end = Instant::now();
    println!("Duration: {:?}", end - start);
}

fn test_triangles() {
    let width: usize = 128;
    let height: usize = 128;
    let mut img = Image::new(width, height);

    // Trianlge 1
    let point_a = Point { x: 7, y: 45 };
    let point_b = Point { x: 35, y: 100 };
    let point_c = Point { x: 45, y: 60 };

    // Triangle 2
    let point_d = Point { x: 120, y: 35 };
    let point_e = Point { x: 90, y: 5 };
    let point_f = Point { x: 45, y: 110 };

    // Triangle 3
    let point_g = Point { x: 115, y: 83 };
    let point_h = Point { x: 80, y: 90 };
    let point_i = Point { x: 85, y: 120 };
    draw_triangle(&point_a, &point_b, &point_c, &mut img, Color::Red);
    draw_triangle(&point_d, &point_e, &point_f, &mut img, Color::White);
    draw_triangle(&point_g, &point_h, &point_i, &mut img, Color::Green);

    img.write_to_file("triangles.tga", true, true);
}

fn test_obj_files() {
    let head_path = Path::new("./assets/head.obj");
    let body_path = Path::new("./assets/body.obj");
    let diablo_path = Path::new("./assets/diablo.obj");
    let path = diablo_path;

    let width: usize = 1600;
    let height: usize = 1600;
    let mut img = Image::new(width, height);

    if let Ok(diablo_model) = parse_obj_file(path) {
        let draw_res = draw_obj_file(diablo_model, &mut img);
        match draw_res {
            Ok(_) => {
                img.write_to_file("model.tga", true, true);
            }
            Err(e) => {
                eprintln!("Failed to render obj object: {:?}", e);
            }
        };
    };
}
