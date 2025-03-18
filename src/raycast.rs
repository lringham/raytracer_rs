use crate::vec3f::Vec3f;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3f,
    pub direction: Vec3f,
}

impl Ray {
    pub fn new(origin: Vec3f, direction: Vec3f) -> Self {
        Ray { origin, direction }
    }

    pub fn at_length(&self, length: f32) -> Vec3f {
        self.origin + self.direction * length
    }

    pub fn reflect(&self, position: &Vec3f, normal: &Vec3f) -> Ray {
        let new_dir = (self.origin - position).normalized().reflected(normal);
        Ray::new(*position, new_dir)
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct RaycastResult {
    pub distance: f32,
    pub position: Vec3f,
    pub normal: Vec3f,
    pub geom_idx: usize,
}

impl RaycastResult {
    pub fn new(distance: f32, position: Vec3f, normal: Vec3f) -> Self {
        RaycastResult {
            distance,
            position,
            normal,
            geom_idx: 0,
        }
    }
}

pub trait Raycastable {
    fn raycast(&self, ray: &Ray) -> Option<RaycastResult>;
}
