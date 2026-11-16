#[cfg(debug_assertions)]
pub fn draw_z_buffer(z_buff: &[Vec<f64>], width: usize, height: usize) {
    use crate::tga::{Grayscale, Image};

    let enabled = false;
    if !enabled {
        return;
    }
    let mut z_buff_img = Image::<Grayscale>::new(width, height);
    for (i, row) in z_buff.iter().enumerate() {
        for (j, z_value_f64) in row.iter().enumerate() {
            // This might break one day, idk. Seems like an aggressive cast
            let depth_color = Grayscale {
                i: ((*z_value_f64 + 1.) * 100.) as u8,
            };
            let _ = z_buff_img.set_pixel(i, j, depth_color);
        }
    }

    let _ = z_buff_img.write_to_file("z_buffer.tga", true, false);
}
