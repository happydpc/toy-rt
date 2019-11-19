use crate::prelude::{Material, Hit, AABB, HitRecord, Ray, Vec3};

pub struct MovingSphere<T: Material> {
    pub center0: Vec3,
    pub center1: Vec3,
    pub time0: f32,
    pub time1: f32,
    pub radius: f32,
    pub material: T,
}

impl<T: Material> MovingSphere<T> {
    fn center(&self, time: f32) -> Vec3 {
        self.center0 + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl<T: Material> Hit for MovingSphere<T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        let oc = ray.origin - self.center(ray.time);
        let a = Vec3::dot(ray.direction, ray.direction);
        let b = Vec3::dot(oc, ray.direction);
        let c = Vec3::dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0. {
            let disc_sqrt = discriminant.sqrt();

            for &solution in &[(-b - disc_sqrt) / a, (-b + disc_sqrt) / a] {
                if solution < t_max && solution > t_min {
                    let p = ray.point_at_parameter(solution);
                    let normal = (p - self.center(ray.time)) / self.radius;
                    return Some(HitRecord { t: solution, p, normal, mat: &self.material, u: 0., v: 0. })
                }
            }
        }

        None
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        let radius = self.radius;

        let box0 = AABB {
            min: self.center(t0) - Vec3::splat(radius),
            max: self.center(t0) + Vec3::splat(radius),
        };
        let box1 = AABB {
            min: self.center(t1) - Vec3::splat(radius),
            max: self.center(t1) + Vec3::splat(radius),
        };

        Some(AABB::surrounding_box(box0, box1))
    }
}
