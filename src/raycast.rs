use crate::{geometry, vec3f};

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
    fn raycast(&self, ray: &Ray) -> Option<RaycastResult>;
}

impl Raycastable for geometry::Sphere {
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

impl Raycastable for geometry::Geometry {
    fn raycast(&self, ray: &Ray) -> Option<RaycastResult> {
        match self {
            geometry::Geometry::Sphere(sphere) => sphere.raycast(ray),
        }
    }
}
