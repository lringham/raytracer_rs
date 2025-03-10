use crate::{geometry, vec3f};

#[allow(dead_code)]
pub struct RaycastResult {
    pub distance: f32,
    pub hit: vec3f::Vec3f,
    pub normal: vec3f::Vec3f,
}

impl RaycastResult {
    pub fn new(distance: f32, hit: vec3f::Vec3f, normal: vec3f::Vec3f) -> Self {
        RaycastResult {
            distance,
            hit,
            normal,
        }
    }
}

pub trait Raycastable {
    fn raycast(&self, ray: &geometry::Ray) -> Option<RaycastResult>;
}

impl Raycastable for geometry::Sphere {
    fn raycast(&self, ray: &geometry::Ray) -> Option<RaycastResult> {
        let radius = self.radius;
        let radius2 = radius * radius;
        let s = self.center - ray.origin;
        let a = ray.direction.dot(&s);
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
