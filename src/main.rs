use rand::Rng;
use image::{ImageBuffer};
pub mod vec3;
use vec3::Vec3;
pub mod ray;
use ray::Ray;
pub mod hitable;
use hitable::Hit;
pub mod sphere;
use sphere::Sphere;
pub mod camera;
use camera::Camera;

pub fn chapter1() {
    let img = ImageBuffer::from_fn(200, 100, |x, y| {
        let r = x as f32 / 200.;
        let g = (99-y) as f32 / 100.;
        let b = 0.2;
        let ir = (256f32*r).floor() as u8;
        let ig = (256f32*g).floor() as u8;
        let ib = (256f32*b).floor() as u8;
        image::Rgb([ir, ig, ib])
    });
    img.save("test.png").expect("Unable to write image.");
}

pub fn chapter2() {
    let img = ImageBuffer::from_fn(200, 100, |x, y| {
        let v = Vec3::new(x as f32 / 200., (99-y) as f32 / 100., 0.2);
        let ir = (256f32*v.x).floor() as u8;
        let ig = (256f32*v.y).floor() as u8;
        let ib = (256f32*v.z).floor() as u8;
        image::Rgb([ir, ig, ib])
    });
    img.save("test.png").expect("Unable to write image.");
}

fn color(r: &Ray<f32>) -> [u8; 3] {
    let mut unit_direction = r.direction();
    unit_direction.normalize();
    let t = (unit_direction.y + 1.0) / 2.;
    let c = Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
    c.rgb().unwrap()
}

pub fn chapter3() {
    let lower_left = Vec3::new(-2., -1., -1.);
    let horizontal = Vec3::new(4., 0., 0.);
    let vertical = Vec3::new(0., 2., 0.);
    let origin = Vec3::new(0., 0., 0.);
    let img = ImageBuffer::from_fn(200, 100, |x, y| {
        let u = x as f32 / 200.;
        let v = (99-y) as f32 / 100.;
        let r = Ray::new(origin, lower_left + horizontal*u + vertical*v);
        image::Rgb(color(&r))
    });
    img.save("test.png").expect("Unable to write image.");
}

fn hit_sphere(center: Vec3<f32>, radius: f32, ray: &Ray<f32>) -> bool {
    let oc = ray.origin() - center;
    let a = ray.direction().dot(&ray.direction());
    let b = 2.0 * oc.dot(&ray.direction());
    let c = oc.dot(&oc) - radius*radius;
    let discr = b*b - 4.0 * a * c;
    discr > 0.
}

fn color4(r: &Ray<f32>) -> [u8; 3] {
    if hit_sphere(Vec3::new(0., 0., -1.), 0.5, r) {
        Vec3::new(1., 0., 0.).rgb().unwrap()
    } else {
        let mut unit_direction = r.direction();
        unit_direction.normalize();
        let t = (unit_direction.y + 1.0) / 2.;
        let c = Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
        c.rgb().unwrap()
    }
}

pub fn chapter4() {
    let lower_left = Vec3::new(-2., -1., -1.);
    let horizontal = Vec3::new(4., 0., 0.);
    let vertical = Vec3::new(0., 2., 0.);
    let origin = Vec3::new(0., 0., 0.);
    let img = ImageBuffer::from_fn(200, 100, |x, y| {
        let u = x as f32 / 200.;
        let v = (99-y) as f32 / 100.;
        let r = Ray::new(origin, lower_left + horizontal*u + vertical*v);
        image::Rgb(color4(&r))
    });
    img.save("test.png").expect("Unable to write image.");
}

fn color5(r: &Ray<f32>, world: &[Sphere<f32>]) -> [u8; 3] {
    match world.hit(r, 0., std::f32::MAX) {
        Some(hr) => {
            ((Vec3::new(1.0, 1.0, 1.0) + hr.normal) / 2.).rgb().unwrap()
        }
        None => {
            let mut unit_direction = r.direction();
            unit_direction.normalize();
            let t = (unit_direction.y + 1.0) / 2.;
            let c = Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
            c.rgb().unwrap()
        }
    }
}

pub fn chapter5() {
    let lower_left = Vec3::new(-2., -1., -1.);
    let horizontal = Vec3::new(4., 0., 0.);
    let vertical = Vec3::new(0., 2., 0.);
    let origin = Vec3::new(0., 0., 0.);
    let world = vec![
        Sphere::new(Vec3::new(0., 0., -1.), 0.5),
        Sphere::new(Vec3::new(0., -100.5, -1.), 100.),
    ];
    let img = ImageBuffer::from_fn(200, 100, |x, y| {
        let u = x as f32 / 200.;
        let v = (99-y) as f32 / 100.;
        let r = Ray::new(origin, lower_left + horizontal*u + vertical*v);
        image::Rgb(color5(&r, &world))
    });
    img.save("test.png").expect("Unable to write image.");
}

fn color6(r: &Ray<f32>, world: &[Sphere<f32>]) -> Vec3<f32> {
    match world.hit(r, 0., std::f32::MAX) {
        Some(hr) => {
            (Vec3::new(1.0, 1.0, 1.0) + hr.normal) / 2.
        }
        None => {
            let mut unit_direction = r.direction();
            unit_direction.normalize();
            let t = (unit_direction.y + 1.0) / 2.;
            Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
        }
    }
}

pub fn chapter6() {
    let world = vec![
        Sphere::new(Vec3::new(0., 0., -1.), 0.5),
        Sphere::new(Vec3::new(0., -100.5, -1.), 100.),
    ];
    Camera::default()
        .image(|ray| color6(&ray, &world), "test.png");
}

fn rand_in_unit_ball() -> Vec3<f32> {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vec3::from_array(rng.gen::<[f32; 3]>()) * 2. - Vec3::new(1., 1., 1.);
        if p.squared_length() >= 1. {
            return p
        }
    }
}

fn color7(r: &Ray<f32>, world: &[Sphere<f32>]) -> Vec3<f32> {
    match world.hit(r, 0.0001, std::f32::MAX) {
        Some(hr) => {
            let target = hr.p + hr.normal + rand_in_unit_ball();
            color7(&Ray::new(hr.p, target - hr.p), world) / 2.
        }
        None => {
            let mut unit_direction = r.direction();
            unit_direction.normalize();
            let t = (unit_direction.y + 1.0) / 2.;
            Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
        }
    }
}

pub fn chapter7() {
    let world = vec![
        Sphere::new(Vec3::new(0., 0., -1.), 0.5),
        Sphere::new(Vec3::new(0., -100.5, -1.), 100.),
    ];
    Camera::default()
        .image(|ray| color7(&ray, &world), "test.png");
}

fn main() {
    chapter7()
}
