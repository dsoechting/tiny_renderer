use std::{path::Path, time::Instant};

use tiny_renderer::{
    draw::draw_obj_file,
    obj::parse_obj_file,
    tga::{Image, RGB},
};

fn main() {
    let start = Instant::now();
    test_obj_files();
    let end = Instant::now();
    println!("Duration: {:?}", end - start);
}

fn test_obj_files() {
    let _head_path = Path::new("./assets/head.obj");
    let _body_path = Path::new("./assets/body.obj");
    let diablo_path = Path::new("./assets/diablo.obj");
    let path = diablo_path;
    // let path = head_path;

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
