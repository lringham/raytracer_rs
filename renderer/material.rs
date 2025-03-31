use crate::vec3f::Vec3f;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Material {
    pub color: Vec3f,
}
