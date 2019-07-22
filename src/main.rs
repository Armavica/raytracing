use clap::{App, Arg};
use num_traits::identities::Zero;
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
pub mod material;
use material::Material;

fn color(r: &Ray<f32>, world: &[Sphere<f32>], depth: usize) -> Vec3<f32> {
    match world.hit(r, 0.0001, std::f32::MAX) {
        Some(hr) => {
            if depth < 50 {
                if let Some((attenuation, scattered)) = hr.material.scatter(r, &hr) {
                    attenuation * color(&scattered, world, depth+1)
                } else {
                    Vec3::zero()
                }
            } else {
                Vec3::zero()
            }
        }
        None => {
            let unit_direction = r.direction().unit();
            let t = (unit_direction.y + 1.0) / 2.;
            Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
        }
    }
}

fn main() {
    let matches = App::new("raytracing")
        .arg(Arg::with_name("nbsamples")
             .short("s")
             .help("Number of samples for each pixel")
             .takes_value(true))
        .arg(Arg::with_name("width")
             .short("w")
             .help("Width of the image in pixels")
             .takes_value(true))
        .arg(Arg::with_name("height")
             .short("h")
             .help("Height of the image in pixels")
             .takes_value(true))
        .arg(Arg::with_name("output")
             .short("o")
             .help("Output file")
             .takes_value(true))
        .get_matches();
    let height = matches.value_of("height").unwrap_or("512").parse().unwrap();
    let width = matches.value_of("width").unwrap_or("1024").parse().unwrap();
    let nbsamples = matches.value_of("nbsamples").unwrap_or("100").parse().unwrap();
    let output = matches.value_of("output").unwrap_or("test.png");
    let world = vec![
        Sphere::new(Vec3::new(0., 0., -1.), 0.5, Material::Lambertian(Vec3::new(0.1, 0.2, 0.5))),
        Sphere::new(Vec3::new(0., -100.5, -1.), 100., Material::Lambertian(Vec3::new(0.8, 0.8, 0.0))),
        Sphere::new(Vec3::new(1., 0., -1.), 0.5, Material::Metal(Vec3::new(0.8, 0.6, 0.2), 0.3)),
        Sphere::new(Vec3::new(-1., 0., -1.), 0.5, Material::Dielectric(1.5)),
        Sphere::new(Vec3::new(-1., 0., -1.), -0.45, Material::Dielectric(1.5)),
    ];
    Camera::default()
        .size(width, height)
        .nbsamples(nbsamples)
        .image(|ray| color(&ray, &world, 0), output);
}
