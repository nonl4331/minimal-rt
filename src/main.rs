mod util;
use util::*;

fn main() {
    let mut img = vec![Vec3::ZERO; 1024 * 1024];
    for (i, px) in img.iter_mut().enumerate() {
        let x = i % 1024;
        let y = i / 1024;
        *px = Vec3::new(x as f32 / 1024.0, y as f32 / 1024.0, 1.0);
    }
    save_image("test.png", &img, 1024);
}

fn save_image(name: &str, img: &[Vec3], width: usize) {
    let height = img.len() / width;

    let mut imgbuf = image::ImageBuffer::new(width as u32, height as u32);

    let to_u8 = |v: f32| -> u8 { (v.powf(1.0 / 2.2) * 255.999) as u8 };

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Rgb([
            to_u8(x as f32 / width as f32),
            to_u8(y as f32 / height as f32),
            255,
        ]);
    }

    imgbuf.save(name).unwrap();
}
