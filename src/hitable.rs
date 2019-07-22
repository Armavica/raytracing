use num_traits::Float;
use std::cmp;
use std::cmp::PartialOrd;
use std::cmp::PartialEq;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::Material;

pub struct HitRecord<F: Float> {
    pub t: F,
    pub p: Vec3<F>,
    pub normal: Vec3<F>,
    pub material: Material<F>,
}

impl<F: Float> PartialEq for HitRecord<F> {
    fn eq(&self, other: &Self) -> bool {
        self.t.eq(&other.t)
    }
}

impl<F: Float> Eq for HitRecord<F> {}

impl<F: Float> PartialOrd for HitRecord<F> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.t.partial_cmp(&other.t)
    }
}

impl<F: Float> Ord for HitRecord<F> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.t.partial_cmp(&other.t).unwrap()
    }
}

impl<F: Float> HitRecord<F> {
    pub fn new(t: F, p: Vec3<F>, normal: Vec3<F>, material: Material<F>) -> Self {
        HitRecord { t, p, normal, material }
    }
}

pub trait Hit {
    type F: Float;
    fn hit(&self, r: &Ray<Self::F>, t_min: Self::F, t_max: Self::F) -> Option<HitRecord<Self::F>>;
}

impl<S: Hit> Hit for &[S] {
    type F = S::F;
    fn hit(&self, r: &Ray<Self::F>, t_min: Self::F, t_max: Self::F) -> Option<HitRecord<Self::F>> {
        self.iter().filter_map(|obj| obj.hit(r, t_min, t_max)).min()
    }
}

