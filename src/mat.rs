use crate::prelude::*;

const EP: f32 = 0.00001;

pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Material {
    pub fn scatter(&self, ray: &mut Ray, sect: &Intersection, rng: &mut impl Rng) {
        match self {
            Self::Lambertian(m) => Lambertian::scatter(ray, sect, rng),
            Self::Metal(_) => todo!(),
            Self::Dielectric(_) => todo!(),
        }
    }
    pub fn eval(&self, _wo: Vec3, _wi: Vec3) -> Vec3 {
        match self {
            Self::Lambertian(m) => m.albedo,
            Self::Metal(_) => todo!(),
            Self::Dielectric(_) => todo!(),
        }
    }
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
    pub fn scatter(ray: &mut Ray, sect: &Intersection, rng: &mut impl Rng) {
        *ray = Ray::new(sect.pos + sect.nor * EP, sect.nor + random_unit_vec(rng));
    }
}

pub struct Metal {
    colour: Vec3,
    fuzz: f32,
}

pub struct Dielectric {
    ior: f32,
}

fn random_unit_vec(rng: &mut impl Rng) -> Vec3 {
    loop {
        let p = Vec3::new(
            rng.random_range(-1.0..1.0),
            rng.random_range(-1.0..1.0),
            rng.random_range(-1.0..1.0),
        );
        let mag = p.mag_sq();
        if mag <= 1.0 {
            return p * (1.0 / mag.sqrt());
        }
    }
}
