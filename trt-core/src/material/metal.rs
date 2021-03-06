use crate::prelude::{Vec3, Material, Ray, HitRecord};
use crate::utils::{reflect, random_in_unit_sphere};

pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = reflect(r_in.direction.unit(), rec.normal);
        let scattered = Ray {
            origin: rec.p,
            direction: reflected + self.fuzz * random_in_unit_sphere(rand::thread_rng()),
            time: 0.
        };
        let attenuation = self.albedo;
        if Vec3::dot(scattered.direction, rec.normal) > 0. {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}
