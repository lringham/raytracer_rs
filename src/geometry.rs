use crate::{
    raycast::{Ray, RaycastResult, Raycastable},
    vec3f,
};

use serde::{Deserialize, Serialize};

// Sphere
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

impl Raycastable for Sphere {
    fn raycast(&self, ray: &Ray) -> Option<RaycastResult> {
        let radius = self.radius;
        let radius2 = radius * radius;
        let s = self.center - ray.origin;
        let a = ray.direction.dot(&s);

        if a < 0.0 {
            return None;
        }

        let o2 = s.dot(&s) - a * a;
        if o2 > radius2 {
            return None;
        }

        let x = (radius2 - o2).sqrt();

        let distance = a - x;
        let hit = ray.at_length(distance);
        let normal = (hit - self.center).normalized();

        Some(RaycastResult::new(distance, hit, normal))
    }
}

// Geometry
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Geometry {
    Sphere(Sphere),
}

impl Raycastable for Geometry {
    fn raycast(&self, ray: &Ray) -> Option<RaycastResult> {
        match self {
            Geometry::Sphere(sphere) => sphere.raycast(ray),
        }
    }
}
