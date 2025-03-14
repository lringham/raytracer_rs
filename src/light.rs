use serde::{Deserialize, Serialize};

use crate::vec3f::Vec3f;

#[derive(Serialize, Deserialize, Debug)]
pub struct PointLight {
    pub position: Vec3f,
    pub color: Vec3f,
}


// Geometry
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Light {
    PointLight(PointLight),
}

impl Light {
    pub fn light_vector(&self, origin: &Vec3f) -> Vec3f {
        match self {
            Light::PointLight(point) => (point.position - origin).normalized(),
        }
    }

    pub fn color(&self) -> Vec3f {
        match self {
            Light::PointLight(point) => point.color,
        }
    }
}