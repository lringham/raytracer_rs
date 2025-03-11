use serde::{Deserialize, Serialize};

use crate::{raycast::Ray, vec3f::Vec3f};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Camera {
    pub position: Vec3f,
    pub direction: Vec3f,
    pub fovy: f32,
    pub resolution: (usize, usize),
}

impl Camera {
    pub fn get_ray(&self, x: usize, y: usize) -> Ray {
        let dir = Vec3f::new(
            0.01 * (x as i32 - self.resolution.0 as i32 / 2) as f32,
            0.01 * (y as i32 - self.resolution.1 as i32 / 2) as f32,
            -1.0,
        )
        .normalized();
        Ray::new(self.position, dir)
    }
}
