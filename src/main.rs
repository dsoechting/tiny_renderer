use std::{path::Path, time::Instant};

use tiny_renderer::{
    colors::Color,
    draw::draw_obj_file,
    math::Vector3,
    obj::parse_obj_file,
    tga::{Image, RGB, RGBA},
    triangle::Triangle,
};

fn main() {
    let start = Instant::now();
    test_obj_files();
    // test_triangles();
    let end = Instant::now();
    println!("Duration: {:?}", end - start);
}

fn test_triangles() {
    let width: usize = 128;
    let height: usize = 128;
    let mut img = Image::<RGBA>::new(width, height);

    // Trianlge 1
    let vector_a = Vector3::new([7, 45, 0]);
    let vector_b = Vector3::new([35, 100, 0]);
    let vector_c = Vector3::new([45, 60, 0]);
    let triangle_1 = Triangle {
        vector_a,
        vector_b,
        vector_c,
    };

    // Triangle 2
    let vector_d = Vector3::new([120, 35, 0]);
    let vector_e = Vector3::new([90, 5, 0]);
    let vector_f = Vector3::new([45, 110, 0]);
    let triangle_2 = Triangle {
        vector_a: vector_d,
        vector_b: vector_e,
        vector_c: vector_f,
    };

    // Triangle 3
    let vector_g = Vector3::new([115, 83, 0]);
    let vector_h = Vector3::new([80, 90, 0]);
    let vector_i = Vector3::new([85, 120, 0]);
    let triangle_3 = Triangle {
        vector_a: vector_g,
        vector_b: vector_h,
        vector_c: vector_i,
    };
    let _ = triangle_1.draw(Color::Red.rgba_value(), &mut img, None);
    let _ = triangle_2.draw(Color::White.rgba_value(), &mut img, None);
    let _ = triangle_3.draw(Color::Green.rgba_value(), &mut img, None);

    let _ = img.write_to_file("triangles.tga", true, true);
}

fn test_obj_files() {
    let _head_path = Path::new("./assets/head.obj");
    let _body_path = Path::new("./assets/body.obj");
    let diablo_path = Path::new("./assets/diablo.obj");
    let path = diablo_path;
    // let path = body_path;

    let width: usize = 1600;
    let height: usize = 1600;
    let mut img = Image::<RGB>::new(width, height);

    if let Ok(model) = parse_obj_file(path) {
        let draw_res = draw_obj_file(model, &mut img);

        match draw_res {
            Ok(_) => {
                let _ = img.write_to_file("model.tga", true, true);
            }
            Err(e) => {
                eprintln!("Failed to render obj object: {:?}", e);
            }
        };
    };
}
