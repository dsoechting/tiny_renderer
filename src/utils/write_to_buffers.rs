use crate::tga::{ColorSpace, Image};

use anyhow::Result;

pub fn write_to_buffers<T: ColorSpace + Copy>(
    z_buffer: &mut [Vec<f64>],
    img: &mut Image<T>,
    x: usize,
    y: usize,
    z: f64,
    color: T,
) -> Result<()> {
    if x >= img.width || y >= img.height {
        return Ok(());
    }
    if z > z_buffer[x][y] {
        z_buffer[x][y] = z;
        img.set_pixel(x, y, color)?;
    }
    Ok(())
}
