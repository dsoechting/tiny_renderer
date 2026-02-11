use crate::{
    math::Vector3,
    tga::{ColorSpace, Image},
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
            vector_a: vector_a.clone(),
            vector_b: vector_b.clone(),
            vector_c: vector_c.clone(),
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
        mut z_buff_opt: Option<&mut Vec<Vec<f64>>>,
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

                // Filter out negative values, they're off screen
                if let (Ok(x_unsigned), Ok(y_unsigned)) = (usize::try_from(x), usize::try_from(y)) {
                    if let Some(z_buffer) = z_buff_opt.as_deref_mut() {
                        // Direct index feels risky, but it should be safe.
                        if z > z_buffer[x_unsigned][y_unsigned] {
                            z_buffer[x_unsigned][y_unsigned] = z;
                            img.set_pixel(x_unsigned, y_unsigned, color)?;
                        }
                    } else {
                        img.set_pixel(x_unsigned, y_unsigned, color)?;
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{
        colors::Color,
        math::Vector3,
        tga::{Image, RGBA},
        triangle::Triangle,
    };

    #[test]
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
}
