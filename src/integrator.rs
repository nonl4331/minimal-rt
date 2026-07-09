use crate::prelude::*;

const MAX_DEPTH: u64 = 50;
const RUSSIAN_ROULETTE_THRESHOLD: u64 = 15;

pub struct Naive {}
impl Naive {
    pub fn rgb(mut ray: Ray, world: &World, rng: &mut impl Rng) -> (Vec3, u64) {
        let (mut tp, mut rgb) = (Vec3::ONE, Vec3::ZERO);

        let mut depth = 0;

        while depth < MAX_DEPTH {
            depth += 1;
            let sect = get_intersection(&ray, world);

            if sect.is_none() {
                let a = 0.5 * (ray.dir.y + 1.0);
                rgb += tp * (a * Vec3::new(0.5, 0.7, 1.0) + (1.0 - a) * Vec3::ONE);
                break;
            }

            let mat = &world.materials[sect.mat];

            let wo = -ray.dir;

            mat.scatter(&mut ray, &sect, rng);

            tp *= mat.eval(wo, ray.dir);

            if depth > RUSSIAN_ROULETTE_THRESHOLD {
                let p = tp.component_max();
                if rng.random::<f32>() > p {
                    break;
                }
                tp *= 1.0 / p;
            }
        }
        (rgb, depth)
    }
}

fn get_intersection(ray: &Ray, world: &World) -> Intersection {
    let mut int = Intersection::NONE;
    for s in &world.objects {
        let new_int = s.get_intersection(ray);
        if int.is_none() || new_int.t <= int.t && new_int.t > 0.0 {
            int = new_int;
        }
    }
    int
}
