use crate::{
    math::Vector3,
    tga::{ColorSpace, Grayscale, Image},
};
use anyhow::Result;

#[derive(Debug)]
pub struct Triangle {
    pub vector_a: Vector3<isize>,
    pub vector_b: Vector3<isize>,
    pub vector_c: Vector3<isize>,
}

impl Triangle {
    pub fn create_from_refs(
        vector_a: &Vector3<isize>,
        vector_b: &Vector3<isize>,
        vector_c: &Vector3<isize>,
    ) -> Self {
        Triangle {
            vector_a: *vector_a,
            vector_b: *vector_b,
            vector_c: *vector_c,
        }
    }
    pub fn area(&self) -> f64 {
        0.5 * ((self.vector_b.y() - self.vector_a.y()) * (self.vector_b.x() + self.vector_a.x())
            + (self.vector_c.y() - self.vector_b.y()) * (self.vector_c.x() + self.vector_b.x())
            + (self.vector_a.y() - self.vector_c.y()) * (self.vector_a.x() + self.vector_c.x()))
            as f64
    }

    pub fn draw<T: ColorSpace + Copy>(
        &self,
        color: T,
        img: &mut Image<T>,
        mut z_buffer_opt: Option<&mut Image<Grayscale>>,
    ) -> Result<()> {
        let bb_min_x = self
            .vector_a
            .x()
            .min(self.vector_b.x())
            .min(self.vector_c.x());
        let bb_max_x = self
            .vector_a
            .x()
            .max(self.vector_b.x())
            .max(self.vector_c.x());
        let bb_min_y = self
            .vector_a
            .y()
            .min(self.vector_b.y())
            .min(self.vector_c.y());
        let bb_max_y = self
            .vector_a
            .y()
            .max(self.vector_b.y())
            .max(self.vector_c.y());

        let total_area = self.area();

        // Figure out how to do this in parallel
        // Will probably need to adjust the Image module
        for y in bb_min_y..=bb_max_y {
            for x in bb_min_x..=bb_max_x {
                let current_point = Vector3::new([x, y, 0]);
                let alpha: f64 =
                    Triangle::create_from_refs(&current_point, &self.vector_b, &self.vector_c)
                        .area()
                        / total_area;
                let beta: f64 =
                    Triangle::create_from_refs(&current_point, &self.vector_c, &self.vector_a)
                        .area()
                        / total_area;
                let gamma: f64 =
                    Triangle::create_from_refs(&current_point, &self.vector_a, &self.vector_b)
                        .area()
                        / total_area;
                if alpha.is_sign_negative() || beta.is_sign_negative() || gamma.is_sign_negative() {
                    continue;
                }
                let z = alpha * self.vector_a.z() as f64
                    + beta * self.vector_b.z() as f64
                    + gamma * self.vector_c.z() as f64;
                let depth_color = Grayscale { i: z as u8 };
                let x_unsigned = usize::try_from(x)?;
                let y_unsigned = usize::try_from(y)?;

                if let Some(z_buffer) = z_buffer_opt.as_deref_mut() {
                    if let Some(z_depth) = z_buffer.get_pixel(x_unsigned, y_unsigned)
                        && depth_color.i > z_depth.i
                    {
                        z_buffer.set_pixel(x_unsigned, y_unsigned, depth_color)?;
                        img.set_pixel(x_unsigned, y_unsigned, color)?;
                    }
                } else {
                    img.set_pixel(x_unsigned, y_unsigned, color)?;
                }
            }
        }
        Ok(())
    }
}
