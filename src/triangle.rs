use crate::{
    colors::Color,
    tga::{Image, RGBA},
    types::Point,
};
use anyhow::Result;

#[derive(Debug)]
pub struct Triangle {
    pub point_a: Point,
    pub point_b: Point,
    pub point_c: Point,
}

impl Triangle {
    pub fn create_from_refs(point_a: &Point, point_b: &Point, point_c: &Point) -> Self {
        Triangle {
            point_a: point_a.clone(),
            point_b: point_b.clone(),
            point_c: point_c.clone(),
        }
    }
    pub fn area(&self) -> f64 {
        0.5 * ((self.point_b.y - self.point_a.y) * (self.point_b.x + self.point_a.x)
            + (self.point_c.y - self.point_b.y) * (self.point_c.x + self.point_b.x)
            + (self.point_a.y - self.point_c.y) * (self.point_a.x + self.point_c.x))
            as f64
    }

    pub fn draw(&self, color: Color, img: &mut Image<RGBA>) -> Result<()> {
        let bb_min_x = self.point_a.x.min(self.point_b.x).min(self.point_c.x);
        let bb_max_x = self.point_a.x.max(self.point_b.x).max(self.point_c.x);
        let bb_min_y = self.point_a.y.min(self.point_b.y).min(self.point_c.y);
        let bb_max_y = self.point_a.y.max(self.point_b.y).max(self.point_c.y);

        let total_area = self.area();
        // z index hack
        if total_area < 1.0 {
            return Ok(());
        }

        // Figure out how to do this in parallel
        // Will probably need to adjust the Image module
        for y in bb_min_y..=bb_max_y {
            for x in bb_min_x..=bb_max_x {
                let current_point = Point { x, y };
                let alpha: f64 =
                    Triangle::create_from_refs(&current_point, &self.point_b, &self.point_c).area()
                        / total_area;
                let beta: f64 =
                    Triangle::create_from_refs(&current_point, &self.point_c, &self.point_a).area()
                        / total_area;
                let gamma: f64 =
                    Triangle::create_from_refs(&current_point, &self.point_a, &self.point_b).area()
                        / total_area;
                if alpha.is_sign_negative() || beta.is_sign_negative() || gamma.is_sign_negative() {
                    continue;
                }
                let x_unsigned = usize::try_from(x)?;
                let y_unsigned = usize::try_from(y)?;
                img.set(x_unsigned, y_unsigned, color.rgba_value())?;
            }
        }
        Ok(())
    }
}
