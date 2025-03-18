use crate::{
    camera::Camera,
    geometry::Geometry,
    light::Light,
    material::Material,
    raycast::{Ray, RaycastResult, Raycastable},
    vec3f::Vec3f,
};
use collections::HashMap;
use std::collections;
use std::{fs::File, io::BufReader};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Scene {
    pub bg_color: Vec3f,
    pub camera: Camera,
    pub materials: HashMap<String, Material>,
    pub geometry: Vec<Geometry>,
    pub lights: Vec<Light>,
    pub material_map: HashMap<String, Vec<usize>>,
}

impl Scene {
    pub fn from(filename: &str) -> Option<Scene> {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let scene: Scene = serde_json::from_reader(reader).unwrap();
        Some(scene)
    }

    pub fn render(&self, ray: &Ray, max_iters: usize) -> Vec3f {
        if max_iters == 0 {
            Vec3f::new(0.0, 0.0, 0.0)
        } else if let Some(hit) = self.trace(ray) {            
            self.shade(&hit) + self.render(&ray.reflect(&hit.position, &hit.normal), max_iters - 1) * 0.2
        } else {
            self.bg_color
        }
    }

    fn get_material(&self, id: usize) -> Option<Material> {
        for (name, indices) in self.material_map.iter() {
            if indices.contains(&id) {
                return self.materials.get(name).cloned();
            }
        }
        None
    }

    fn trace(&self, ray: &Ray) -> Option<RaycastResult> {
        let mut prev_dist = f32::MAX;
        let mut res: Option<RaycastResult> = None;
        for (i, geom) in self.geometry.iter().enumerate() {
            let mut temp_res = geom.raycast(ray);
            if let Some(ref mut hit) = temp_res {
                if hit.distance < prev_dist {
                    prev_dist = hit.distance;
                    hit.geom_idx = i;
                    res = temp_res;
                }
            }
        }
        res
    }

    fn shade(&self, hit: &RaycastResult) -> Vec3f {
        let material = self.get_material(hit.geom_idx).unwrap();
        let v = (self.camera.position - hit.position).normalized();
        let mut color = Vec3f::new(0.0, 0.0, 0.0);
        for light in self.lights.iter() {
            let l = light.light_vector(&hit.position);
            let h = (l + v).normalized();
    
            let ambient = 0.1;
            let lambertian = hit.normal.dot(&l).max(0.0);
            let specular = hit.normal.dot(&h).max(0.0);
            let specular = specular.powi(30);
            color += material.color * (ambient + lambertian) + specular * light.color()
        }
        color
    }
}
