use crate::vec3::Vec3;
use crate::material::Material;
use crate::ray::Ray;
use crate::hit::HitRecord;
use crate::prelude::ParallelTexture;

pub struct Isotropic<T> {
    pub albedo: T
}

impl<T: ParallelTexture> Material for Isotropic<T> {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let scattered = Ray {
            origin: rec.p,
            direction: crate::random_in_unit_sphere(rand::thread_rng()),
            time: r_in.time,
        };
        let attenuation = self.albedo.value(rec.u, rec.v, rec.p);
        Some((scattered, attenuation))
    }
}
