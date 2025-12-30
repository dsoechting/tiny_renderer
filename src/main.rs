use tiny_renderer::tga::{Image, RGBA};

fn main() {
    let width: usize = 64;
    let height: usize = 64;

    let white = RGBA {
        r: 255,
        b: 255,
        g: 255,
        a: 255,
    };
    let mut img = Image::new(width, height);
    img.set(7, 3, white);
    img.set(12, 37, white);
    img.set(62, 53, white);
    img.write_to_file("output.tga", true, true);
}
