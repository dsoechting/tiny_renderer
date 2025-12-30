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
