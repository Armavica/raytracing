use num_traits::Float;
use crate::vec3::Vec3;

pub struct Ray<F: Float> {
    a: Vec3<F>,
    b: Vec3<F>,
}

impl<F: Float> Ray<F> {
    pub fn new(origin: Vec3<F>, direction: Vec3<F>) -> Self {
        Ray { a: origin, b: direction }
    }
}

impl<F: Float> Ray<F> {
    pub fn origin(&self) -> Vec3<F> {
        self.a
    }
    pub fn direction(&self) -> Vec3<F> {
        self.b
    }
    pub fn point_at_parameter(&self, t: F) -> Vec3<F> {
        self.a + self.b*t
    }
}
