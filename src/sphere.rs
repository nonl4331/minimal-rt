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
        let oc = self.centre - ray.origin;
        let a = ray.dir.mag_sq();
        let h = ray.dir.dot(oc);
        let c = oc.mag_sq() - self.radius * self.radius;

        let disc = h * h - a * c;

        if disc < 0.0 {
            return Intersection::NONE;
        }

        let mut t0 = (h - disc.sqrt()) / a;
        let mut t1 = (h + disc.sqrt()) / a;

        if t1 < t0 {
            std::mem::swap(&mut t0, &mut t1);
        };

        let t = if t0 > 0.0 {
            t0
        } else if t1 <= 0.0 {
            return Intersection::NONE;
        } else {
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
