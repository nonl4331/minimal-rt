mod cam;
mod integrator;
mod mat;
mod sphere;
mod util;

mod prelude {
    pub use crate::cam::*;
    pub use crate::mat::*;
    pub use crate::sphere::*;
    pub use crate::util::*;
    pub use rand::{Rng, RngExt};
}

use prelude::*;

const WIDTH: usize = 1200;
const HEIGHT: usize = 675;
const SAMPLES: usize = 10;

fn main() {
    let mut mats = vec![
        Material::Lambertian(Lambertian::new(Vec3::splat(0.5))),
        Material::Dielectric(Dielectric::new(1.5)),
    ];
    let mut obj = vec![Sphere::new(-Vec3::Y * 1000.0, 1000.0, 0)];

    let mut rng = rand::rng();

    for a in -11..11 {
        for b in -11..11 {
            let centre = Vec3::new(
                a as f32 + 0.9 * rng.random::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.random::<f32>(),
            );
            if (centre - Vec3::new(4.0, 0.2, 0.0)).mag() > 0.9 {
                let p = rng.random::<f32>();
                if p < 0.8 {
                    let c1 = Vec3::new(rng.random(), rng.random(), rng.random());
                    let c2 = Vec3::new(rng.random(), rng.random(), rng.random());
                    mats.push(Material::Lambertian(Lambertian::new(c1 * c2)));
                    obj.push(Sphere::new(centre, 0.2, mats.len() - 1));
                } else if p < 0.95 {
                    let colour = Vec3::new(rng.random(), rng.random(), rng.random());
                    mats.push(Material::Metal(Metal::new(colour, rng.random())));
                    obj.push(Sphere::new(centre, 0.2, mats.len() - 1));
                } else {
                    obj.push(Sphere::new(centre, 0.2, 1));
                }
            }
        }
    }
    obj.push(Sphere::new(Vec3::Y, 1.0, 1));

    mats.push(Material::Lambertian(Lambertian::new(Vec3::new(
        0.4, 0.2, 0.1,
    ))));
    obj.push(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, mats.len() - 1));

    mats.push(Material::Metal(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)));
    obj.push(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, mats.len() - 1));

    let world = World::new(obj, mats);

    let cam = Cam::new(
        Vec3::new(13.0, 2.0, 3.0),
        Vec3::ZERO,
        Vec3::Y,
        20.0,
        10.0,
        0.6,
        WIDTH,
        HEIGHT,
    );

    let mut img = vec![Vec3::ZERO; WIDTH * HEIGHT];
    let mut rays = 0;
    let start = std::time::Instant::now();
    for (i, px) in img.iter_mut().enumerate() {
        let mut col = Vec3::zero();
        for _ in 0..SAMPLES {
            let ray = cam.get_ray(i as u64, &mut rng);
            let (ncol, depth) = integrator::Naive::rgb(ray, &world, &mut rng);
            col += ncol;
            rays += depth;
        }
        col /= SAMPLES as f32;
        *px = col
    }
    let render_time = start.elapsed();
    println!("took {} seconds", render_time.as_secs_f64());
    println!(
        "{rays} rays @ {:.2} MRay/s",
        (1e-6 * rays as f64) / render_time.as_secs_f64()
    );
    save_image("test.png", &img, WIDTH);
}

fn save_image(name: &str, img: &[Vec3], width: usize) {
    let height = img.len() / width;

    let mut imgbuf = image::ImageBuffer::new(width as u32, height as u32);

    let to_u8 = |v: f32| -> u8 { (v.powf(1.0 / 2.2) * 255.999) as u8 };

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let rgb = img[x as usize + width * y as usize];
        *pixel = image::Rgb([to_u8(rgb.x), to_u8(rgb.y), to_u8(rgb.z)]);
    }

    imgbuf.save(name).unwrap();
}
