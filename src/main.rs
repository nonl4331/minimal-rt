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

fn main() {
    let mats = vec![Material::Lambertian(Lambertian::new(Vec3::splat(0.5)))];
    let obj = vec![
        Sphere::new(-Vec3::Z, 0.5, 0),
        Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, 0),
    ];
    let world = World::new(obj, mats);

    let cam = Cam::new(
        Vec3::ZERO,
        -Vec3::Z,
        Vec3::Y,
        90.0,
        1.0,
        1024,
        1024,
    );

    let mut rng = rand::rng();

    let mut img = vec![Vec3::ZERO; 1024 * 1024];
    let mut rays = 0;
    for (i, px) in img.iter_mut().enumerate() {
        //let x = i % 1024;
        //let y = i / 1024;
        //*px = Vec3::new(x as f32 / 1024.0, y as f32 / 1024.0, 1.0);
        let ray = cam.get_ray(i as u64, &mut rng);
        let (col, depth) = integrator::Naive::rgb(ray, &world, &mut rng);
        rays += depth;
        *px = col
    }
    println!("rays: {rays}");
    save_image("test.png", &img, 1024);
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
