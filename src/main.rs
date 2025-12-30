use tiny_renderer::tga::{Image, RGBA};

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

const WHITE: RGBA = RGBA {
    r: 255,
    b: 255,
    g: 255,
    a: 255,
};

const BLUE: RGBA = RGBA {
    r: 0,
    b: 255,
    g: 0,
    a: 255,
};

const RED: RGBA = RGBA {
    r: 255,
    b: 0,
    g: 0,
    a: 255,
};

const GREEN: RGBA = RGBA {
    r: 0,
    b: 0,
    g: 255,
    a: 255,
};

const YELLOW: RGBA = RGBA {
    r: 255,
    b: 0,
    g: 200,
    a: 255,
};

fn main() {
    let width: usize = 64;
    let height: usize = 64;

    let point1 = Point { x: 7, y: 3 };
    let point2 = Point { x: 12, y: 37 };
    let point3 = Point { x: 62, y: 53 };

    let mut img = Image::new(width, height);
    img.set(point1.x, point1.y, WHITE);
    img.set(point2.x, point2.y, WHITE);
    img.set(point3.x, point3.y, WHITE);
    draw_line(&point1, &point2, &mut img, BLUE);
    draw_line(&point3, &point2, &mut img, GREEN);
    draw_line(&point3, &point1, &mut img, YELLOW);
    draw_line(&point1, &point3, &mut img, RED);
    img.write_to_file("output.tga", true, true);
}
fn draw_line(origin: &Point, destination: &Point, img: &mut Image<RGBA>, color: RGBA) {
    let origin_x = origin.x as f64;
    let origin_y = origin.y as f64;
    let dest_x = destination.x as f64;
    let dest_y = destination.y as f64;

    for x in origin.x..destination.x {
        dbg!(x, origin.x, destination.x);
        //54/
        let t = ((x as f64) - origin_x) / (dest_x - origin_x);
        let line_y = (origin_y + t * (dest_y - origin_y)) as usize;
        dbg!(t, line_y);
        img.set(x, line_y, color);
    }
}

// fn draw_line(origin: &Point, destination: &Point, img: &mut Image<RGBA>, color: RGBA) {
//     let start = 0.0;
//     let end = 1.0;
//     let step = 0.02;
//     let mut current = start;
//
//     let origin_x = origin.x as f64;
//     let origin_y = origin.y as f64;
//     let dest_x = destination.x as f64;
//     let dest_y = destination.y as f64;
//
//     while current < end {
//         let line_x = (origin_x + current * (dest_x - origin_x)) as usize;
//         let line_y = (origin_y + current * (dest_y - origin_y)) as usize;
//         img.set(line_x, line_y, color);
//
//         current += step;
//     }
// }
