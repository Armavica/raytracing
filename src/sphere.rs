use num_traits::Float;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hitable::{Hit, HitRecord};

pub struct Sphere<F: Float> {
    center: Vec3<F>,
    radius: F,
}

impl<F: Float> Sphere<F> {
    pub fn new(center: Vec3<F>, radius: F) -> Self {
        Sphere { center, radius }
    }
}

impl<F: Float> Hit for Sphere<F> {
    type F = F;
    fn hit(&self, ray: &Ray<F>, t_min: F, t_max: F) -> Option<HitRecord<F>> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(&ray.direction());
        let b = oc.dot(&ray.direction());
        let c = oc.dot(&oc) - self.radius*self.radius;
        let discr = b*b - a*c;
        if discr > F::zero() {
            let temp = (-b - (b*b-a*c).sqrt()) / a;
            if t_min < temp && temp < t_max {
                let p = ray.point_at_parameter(temp);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord::new(temp, p, normal))
            }
            let temp = (-b + (b*b-a*c).sqrt()) / a;
            if t_min < temp && temp < t_max {
                let p = ray.point_at_parameter(temp);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord::new(temp, p, normal))
            }
        }
        None
    }
}
