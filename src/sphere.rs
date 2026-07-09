use crate::prelude::*;

pub struct Sphere {
    centre: Vec3,
    radius: f32,
    pub mat: usize,
}

impl Sphere {
    pub fn new(centre: Vec3, radius: f32, mat: usize) -> Self {
        Self {
            centre,
            radius,
            mat,
        }
    }
    pub fn get_intersection(&self, ray: &Ray) -> Intersection {
        let deltap = self.centre - ray.origin;
        let ddp = ray.dir.dot(deltap);
        let deltapdot = deltap.dot(deltap);

        let remedy_term = deltap - ddp * ray.dir;
        let discriminant = self.radius * self.radius - remedy_term.dot(remedy_term);

        if discriminant <= 0.0 {
            return Intersection::NONE;
        }
        let sqrt_val = discriminant.sqrt();

        let q = if ddp > 0.0 {
            ddp + sqrt_val
        } else {
            ddp - sqrt_val
        };

        let mut t0 = q;
        let mut t1 = (deltapdot - self.radius * self.radius) / q;

        if t1 < t0 {
            std::mem::swap(&mut t0, &mut t1);
        };

        let t = if t0 > 0.0 {
            t0
        } else {
            if t1 <= 0.0 {
                return Intersection::NONE;
            }
            t1
        };

        let point = ray.origin + t * ray.dir;

        let mut normal = (point - self.centre) / self.radius;

        let mut out = true;
        if normal.dot(ray.dir) > 0.0 {
            out = false;
            normal = -normal;
        }

        Intersection::new(t, point, normal, out, self.mat)
    }
}
