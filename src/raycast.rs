use crate::vec3f;

#[derive(Debug)]
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
#[derive(Copy, Clone)]
pub struct RaycastResult {
    pub distance: f32,
    pub position: vec3f::Vec3f,
    pub normal: vec3f::Vec3f,
}

impl RaycastResult {
    pub fn new(distance: f32, position: vec3f::Vec3f, normal: vec3f::Vec3f) -> Self {
        RaycastResult {
            distance,
            position,
            normal,
        }
    }
}

pub trait Raycastable {
    fn raycast(&self, ray: &Ray) -> Option<RaycastResult>;
}
