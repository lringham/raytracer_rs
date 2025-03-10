use crate::vec3f;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Sphere {
    pub center: vec3f::Vec3f,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: vec3f::Vec3f, radius: f32) -> Self {
        Sphere { center, radius }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: vec3f::Vec3f,
    pub direction: vec3f::Vec3f,
}

impl Ray {
    pub fn new(origin: vec3f::Vec3f, direction: vec3f::Vec3f) -> Self {
        Ray { origin, direction }
    }

    pub fn at_length(&self, length: f32) -> vec3f::Vec3f {
        self.origin + self.direction * length
    }
}
