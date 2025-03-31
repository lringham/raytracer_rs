use serde::{Deserialize, Serialize};
use std::ops;

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[allow(dead_code)]
impl Vec3f {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3f { x, y, z }
    }

    pub fn set(&mut self, x: f32, y: f32, z: f32) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    pub fn add(&self, other: &Vec3f) -> Vec3f {
        Vec3f {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn sub(&self, other: &Vec3f) -> Vec3f {
        Vec3f {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn dot(&self, other: &Vec3f) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3f) -> Vec3f {
        Vec3f {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn scale(&self, scalar: f32) -> Vec3f {
        Vec3f {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&mut self) {
        let mag = self.magnitude();
        self.x /= mag;
        self.y /= mag;
        self.z /= mag;
    }

    pub fn normalized(&self) -> Vec3f {
        let mag = self.magnitude();
        Vec3f {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }

    pub fn reflect(&mut self, normal: &Vec3f) {
        *self = 2.0 * (normal.dot(self) * normal) - *self;
    }

    pub fn reflected(&self, normal: &Vec3f) -> Vec3f {
        2.0 * (normal.dot(self) * normal) - self
    }

    pub fn refract(&mut self, normal: &Vec3f) {
        *self = 2.0 * (normal.dot(self) * normal) - *self;
    }

    pub fn refracted(&self, normal: &Vec3f) -> Vec3f {
        2.0 * (normal.dot(self) * normal) - self
    }
}

// Operators
impl ops::Add<Vec3f> for Vec3f {
    type Output = Vec3f;
    fn add(self, rhs: Vec3f) -> Vec3f {
        Vec3f::add(&self, &rhs)
    }
}

impl ops::Add<&Vec3f> for Vec3f {
    type Output = Vec3f;
    fn add(self, rhs: &Vec3f) -> Vec3f {
        Vec3f::add(&self, rhs)
    }
}

impl ops::Sub<Vec3f> for Vec3f {
    type Output = Vec3f;
    fn sub(self, rhs: Vec3f) -> Vec3f {
        Vec3f::sub(&self, &rhs)
    }
}

impl ops::Sub<&Vec3f> for Vec3f {
    type Output = Vec3f;
    fn sub(self, rhs: &Vec3f) -> Vec3f {
        Vec3f::sub(&self, rhs)
    }
}

impl ops::Mul<f32> for Vec3f {
    type Output = Vec3f;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3f::scale(&self, rhs)
    }
}

impl ops::Mul<Vec3f> for f32 {
    type Output = Vec3f;
    fn mul(self, rhs: Vec3f) -> Self::Output {
        Vec3f::scale(&rhs, self)
    }
}

impl ops::Mul<f32> for &Vec3f {
    type Output = Vec3f;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3f::scale(self, rhs)
    }
}

impl ops::Mul<&Vec3f> for f32 {
    type Output = Vec3f;
    fn mul(self, rhs: &Vec3f) -> Self::Output {
        Vec3f::scale(rhs, self)
    }
}

impl ops::AddAssign<Vec3f> for Vec3f {
    fn add_assign(&mut self, rhs: Vec3f) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
