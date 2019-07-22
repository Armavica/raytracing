use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
use num_traits::Float;
use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub enum Material<F: Float> {
    Lambertian(Vec3<F>),
    Metal(Vec3<F>, F),
    Dielectric(F),
}

impl<F: Float> Material<F> {
    pub fn schlick(cosine: F, n: F) -> F {
        let mut r0 = (F::one() - n) / (F::one() + n);
        r0 = r0 * r0;
        r0 + (F::one() - r0) * (F::one() - cosine).powf(F::from(5).unwrap())
    }
    pub fn scatter(&self, ray: &Ray<F>, hr: &HitRecord<F>) -> Option<(Vec3<F>, Ray<F>)> {
        match self {
            Material::Lambertian(albedo) => {
                let target = hr.p + hr.normal + Vec3::rand_in_unit_ball();
                let scattered = Ray::new(hr.p, target - hr.p);
                Some((*albedo, scattered))
            }
            Material::Metal(albedo, fuzz) => {
                let reflected = ray.direction().unit().reflect(&hr.normal);
                let scattered = Ray::new(hr.p, reflected + Vec3::rand_in_unit_ball() * *fuzz);
                Some((*albedo, scattered)).filter(|(_, s)| s.dot(&hr.normal) > F::zero())
            }
            Material::Dielectric(n) => {
                let albedo = Vec3::new(F::one(), F::one(), F::one());
                let reflected = ray.direction().reflect(&hr.normal);
                let cos = ray.dot(&hr.normal) / ray.direction().length();
                let (out_normal, ni_over_nt, cos) = if cos > F::zero() {
                    (
                        -hr.normal,
                        *n,
                        (F::one() - *n * *n * (F::one() - cos * cos)).sqrt(),
                    )
                } else {
                    (hr.normal, n.recip(), -cos)
                };
                if let Some(refracted) = ray.direction().refract(&out_normal, ni_over_nt) {
                    if F::from(rand::thread_rng().gen::<f32>()).unwrap()
                        > Material::schlick(cos, *n)
                    {
                        return Some((albedo, Ray::new(hr.p, refracted)));
                    }
                }
                Some((albedo, Ray::new(hr.p, reflected)))
            }
        }
    }
}
