use crate::prelude::*;

const EP: f32 = 0.0001;

pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Material {
    pub fn scatter(&self, ray: &mut Ray, sect: &Intersection, rng: &mut impl Rng) {
        match self {
            Self::Lambertian(_) => Lambertian::scatter(ray, sect, rng),
            Self::Metal(m) => m.scatter(ray, sect, rng),
            Self::Dielectric(m) => m.scatter(ray, sect, rng),
        }
    }
    pub fn eval(&self, _wo: Vec3, _wi: Vec3) -> Vec3 {
        match self {
            Self::Lambertian(m) => m.albedo,
            Self::Metal(m) => m.colour,
            Self::Dielectric(_) => Vec3::ONE,
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
    pub colour: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(colour: Vec3, fuzz: f32) -> Self {
        Self {
            colour,
            fuzz: fuzz.min(1.0),
        }
    }
    pub fn scatter(&self, ray: &mut Ray, sect: &Intersection, rng: &mut impl Rng) {
        let wo = -ray.dir;

        let wi = wo.reflected(sect.nor) + (self.fuzz * random_unit_vec(rng));
        *ray = Ray::new(sect.pos, wi);
    }
}

pub struct Dielectric {
    ior: f32,
}

impl Dielectric {
    pub fn new(ior: f32) -> Self {
        Self { ior }
    }
    pub fn scatter(&self, ray: &mut Ray, sect: &Intersection, rng: &mut impl Rng) {
        let wo = -ray.dir;

        let mut eta1 = 1.0;
        let mut eta2 = self.ior;

        if !sect.out {
            std::mem::swap(&mut eta1, &mut eta2);
        }
        let eta = eta1 / eta2;

        let cosi = wo.dot(sect.nor);

        let r = fresnel_dielectric(eta1, eta2, sect.nor, wo);

        if r >= rng.random() {
            let wi = wo.reflected(sect.nor);
            *ray = Ray::new(sect.pos, wi);
            return;
        }

        let perp = eta * (cosi * sect.nor - wo);
        let para = -(1.0 - perp.mag_sq()).abs().sqrt() * sect.nor;
        let wi = perp + para;
        *ray = Ray::new(sect.pos, wi);
    }
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

// eta1 = outer ior, eta2 = inner ior
pub fn fresnel_dielectric(eta1: f32, eta2: f32, nor: Vec3, wo: Vec3) -> f32 {
    let eta = eta1 / eta2;

    let cosi = wo.dot(nor);

    let sint_sq = eta.powi(2) * (1.0 - cosi.powi(2));
    let is_tir = sint_sq >= 1.0;
    if is_tir {
        return 1.0;
    }

    let cost = (1.0 - sint_sq).sqrt();

    let rs = ((eta1 * cosi - eta2 * cost) / (eta1 * cosi + eta2 * cost)).powi(2);
    let rp = ((eta1 * cost - eta2 * cosi) / (eta1 * cost + eta2 * cosi)).powi(2);

    0.5 * (rs + rp)
}
