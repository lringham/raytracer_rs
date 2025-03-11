use std::{fs::File, io::BufReader};

use crate::{geometry::Geometry, vec3f::Vec3f};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Scene {
    pub bg_color: Vec3f,
    pub camera_pos: Vec3f,
    pub light_pos: Vec3f,
    pub light_col: Vec3f,
    pub material_col: Vec3f,
    pub geometry: Vec<Geometry>,
}

impl Scene {
    pub fn from(filename: &str) -> Option<Scene> {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let scene: Scene = serde_json::from_reader(reader).unwrap();
        Some(scene)
    }
}
