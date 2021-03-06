use crate::{camera::Camera, prelude::{Hit, Vec3, Color}};
use crate::utils::{compute_color, Rng};

pub struct Scene<World> {
    pub camera: Camera,
    pub width: usize,
    pub height: usize,
    pub world: World,
    pub samples_per_px: u32,
    pub rays_per_sample: u32,
    pub ambiant_color: Vec3,
}

impl<World: Hit> Scene<World> {
    pub fn pixel_color(&self, (x, y): (usize, usize), mut rng: impl Rng) -> Color {
        let summed_color = (0..self.samples_per_px)
            .fold(Vec3::splat(0), |current_color, _r| {
                let u = (x as f32 + rng.gen::<f32>()) / self.width as f32;
                let v = (y as f32 + rng.gen::<f32>()) / self.height as f32;

                let ray = self.camera.get_ray(u, v);

                current_color + compute_color(ray, &self.world, self.ambiant_color, self.rays_per_sample as _)
            });

        (summed_color / self.samples_per_px as f32)
            .sqrt()
            .into()
    }
}
