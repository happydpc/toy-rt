use crate::{camera::Camera, prelude::{Hit, Vec3, Color}};
use crate::utils::{compute_color, Rng, thread_rng};

pub struct Scene<World> {
    pub camera: Camera,
    pub width: usize,
    pub height: usize,
    pub world: World,
    pub ray_per_px: usize,
}

impl<World: Hit> Scene<World> {
    pub fn pixel_color(&self, (x, y): (usize, usize)) -> Color {
        let summed_color = (0..self.ray_per_px)
            .fold(Vec3::splat(0), |current_color, _r| {
                let u = (x as f32 + thread_rng().gen::<f32>()) / self.width as f32;
                let v = (y as f32 + thread_rng().gen::<f32>()) / self.height as f32;

                let ray = self.camera.get_ray(u, v);

                current_color + compute_color(&ray, &self.world, 0)
            });

        (summed_color / self.ray_per_px as f32)
            .sqrt()
            .into()
    }
}