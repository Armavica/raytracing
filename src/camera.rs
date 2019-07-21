use std::path::Path;
use image::ImageBuffer;
use num_traits::Float;
use rand::Rng;
use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Camera<F: Float> {
    lower_left_corner: Vec3<F>,
    horizontal: Vec3<F>,
    vertical: Vec3<F>,
    origin: Vec3<F>,
    nx: u32,
    ny: u32,
    ns: usize,
}

impl<F: Float> Camera<F> {
    pub fn new(lower_left_corner: Vec3<F>, horizontal: Vec3<F>, vertical: Vec3<F>, origin: Vec3<F>, nx: u32, ny: u32, ns: usize) -> Self {
        Camera { lower_left_corner, horizontal, vertical, origin, nx, ny, ns }
    }
    pub fn size(mut self, w: u32, h: u32) -> Self {
        self.nx = w;
        self.ny = h;
        self
    }
    pub fn nbsamples(mut self, ns: usize) -> Self {
        self.ns = ns;
        self
    }
    pub fn get_ray(&self, u: F, v: F) -> Ray<F> {
        Ray::new(self.origin, self.lower_left_corner + self.horizontal*u + self.vertical*v - self.origin)
    }
    pub fn image<Fun, P>(&self, color: Fun, path: P)
        where
        Fun: Fn(Ray<F>) -> Vec3<F>,
        P: AsRef<Path>,
    {
        let mut rng = rand::thread_rng();
        let img = ImageBuffer::from_fn(self.nx, self.ny, |x, y| {
            let avg: Vec3<_> = (0..self.ns).map(|_| {
                let u = (x as f32 + rng.gen::<f32>()) / self.nx as f32;
                let v = ((self.ny-y-1) as f32 + rng.gen::<f32>()) / self.ny as f32;
                let r = self.get_ray(F::from(u).unwrap(), F::from(v).unwrap());
                color(r)
            }).sum();
            image::Rgb((avg / F::from(self.ns).unwrap()).sqrt().rgb().unwrap())
        });
        img.save(path).expect("Unable to write image.");
    }
}

impl<F: Float> Default for Camera<F> {
    fn default() -> Self {
        Camera {
            lower_left_corner: Vec3::new(F::from(-2).unwrap(),
                F::from(-1).unwrap(), F::from(-1).unwrap()),
            horizontal: Vec3::new(F::from(4).unwrap(), F::zero(), F::zero()),
            vertical: Vec3::new(F::zero(), F::from(2).unwrap(), F::zero()),
            origin: Vec3::new(F::zero(), F::zero(), F::zero()),
            nx: 1024,
            ny: 512,
            ns: 500,
        }
    }
}

