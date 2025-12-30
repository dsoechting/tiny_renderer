use tiny_renderer::{
    colors::Color,
    line::draw_line,
    tga::{Image, RGBA},
    types::Point,
};

fn main() {
    let width: usize = 64;
    let height: usize = 64;

    let point1 = Point { x: 7, y: 3 };
    let point2 = Point { x: 12, y: 37 };
    let point3 = Point { x: 62, y: 53 };

    let mut img = Image::new(width, height);

    // Triangle
    draw_line(&point1, &point2, &mut img, Color::Blue);
    draw_line(&point3, &point2, &mut img, Color::Green);
    draw_line(&point3, &point1, &mut img, Color::Yellow);
    draw_line(&point1, &point3, &mut img, Color::Red);
    img.set(point1.x, point1.y, Color::White.rgba_value());
    img.set(point2.x, point2.y, Color::White.rgba_value());
    img.set(point3.x, point3.y, Color::White.rgba_value());
    img.write_to_file("output.tga", true, true);
}
// fn draw_line(point_one: &Point, point_two: &Point, img: &mut Image<RGBA>, color: RGBA) {
//     let delta_x = point_one.x.abs_diff(point_two.x);
//     let delta_y = point_one.y.abs_diff(point_two.y);
//     let steep = delta_y > delta_x;
//     let min_x: usize;
//     let max_x: usize;
//     let point_one_x: f64;
//     let point_one_y: f64;
//     let point_two_x: f64;
//     let point_two_y: f64;
//     if steep {
//         min_x = point_one.y.min(point_two.y);
//         max_x = point_one.y.max(point_two.y);
//         point_one_x = point_one.y as f64;
//         point_one_y = point_one.x as f64;
//         point_two_x = point_two.y as f64;
//         point_two_y = point_two.x as f64;
//     } else {
//         min_x = point_one.x.min(point_two.x);
//         max_x = point_one.x.max(point_two.x);
//         point_one_x = point_one.x as f64;
//         point_one_y = point_one.y as f64;
//         point_two_x = point_two.x as f64;
//         point_two_y = point_two.y as f64;
//     }
//
//     for x in min_x..=max_x {
//         let t = ((x as f64 - point_one_x) / (point_two_x - point_one_x)) as f64;
//         let line_y = (point_one_y + t * (point_two_y - point_one_y)) as usize;
//         if steep {
//             img.set(line_y, x, color);
//         } else {
//             img.set(x, line_y, color);
//         }
//     }
// }
