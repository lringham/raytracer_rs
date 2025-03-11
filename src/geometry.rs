use crate::vec3f;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Sphere {
    pub center: vec3f::Vec3f,
    pub radius: f32,
}

#[allow(dead_code)]
impl Sphere {
    pub fn new(center: vec3f::Vec3f, radius: f32) -> Self {
        Sphere { center, radius }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Geometry {
    Sphere(Sphere),
}
