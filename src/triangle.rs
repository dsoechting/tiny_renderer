use crate::math::Vector3;

#[derive(Debug)]
pub struct Triangle {
    pub vector_a: Vector3<f64>,
    pub vector_b: Vector3<f64>,
    pub vector_c: Vector3<f64>,
}

impl Triangle {
    pub fn create_from_refs(
        vector_a: &Vector3<f64>,
        vector_b: &Vector3<f64>,
        vector_c: &Vector3<f64>,
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
    }
}
