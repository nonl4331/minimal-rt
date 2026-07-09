use crate::prelude::*;

pub struct Cam {
    lower_left: Vec3,
    up: Vec3,
    right: Vec3,
    origin: Vec3,
    width: usize,
    height: usize,
    disk_up: Vec3,
    disk_right: Vec3,
}

impl Cam {
    pub fn new(
        origin: Vec3,
        look_at: Vec3,
        mut up: Vec3,
        vfov: f32,
        focus_dist: f32,
        defocus_angle: f32,
        width: usize,
        height: usize,
    ) -> Self {
        let forward = (look_at - origin).normalised();
        up.normalise();
        let aspect_ratio = width as f32 / height as f32;

        let up_mag = 2.0 * (0.5 * vfov.to_radians()).tan() * focus_dist;
        let right_mag = up_mag * aspect_ratio;

        let right = forward.cross(up);
        let up = right.cross(forward) * up_mag;
        let right = right * right_mag;

        let lower_left = origin - 0.5 * right - 0.5 * up + forward * focus_dist;
        let defocus_r = focus_dist * (0.5 * defocus_angle).to_radians().tan();

        let disk_right = right * defocus_r;
        let disk_up = right * defocus_r;

        Self {
            lower_left,
            up,
            right,
            origin,
            width,
            height,
            disk_up,
            disk_right,
        }
    }
    pub fn get_ray(&self, i: u64, rng: &mut impl Rng) -> Ray {
        let (u, v) = (i % self.width as u64, i / self.width as u64);
        let (u, v) = (
            (u as f32 + rng.random::<f32>()) / self.width as f32,
            (v as f32 + rng.random::<f32>()) / self.height as f32,
        );

        let d = random_in_unit_disk(rng);
        let o = self.origin + (d.x * self.disk_right + d.y * self.disk_up);
        let po = self.lower_left + self.right * u + self.up * (1.0 - v);

        Ray::new(
            o,
            po - o,
        )
    }
}

fn random_in_unit_disk(rng: &mut impl Rng) -> Vec3 {
    loop {
        let p = Vec3::new(rng.random::<f32>(), rng.random::<f32>(), 0.0);
        if p.mag_sq() < 1.0 {
            return p;
        }
    }
}
