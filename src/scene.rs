use crate::{
    camera::Camera, geometry::Geometry, light::Light, material::Material, raycast::{Ray, RaycastResult, Raycastable}, vec3f::Vec3f
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

    pub fn trace(&self, ray: &Ray, max_iters: usize) -> Vec3f {       
        // Find nearest 
        let mut prev_dist = f32::MAX;
        let mut color = Vec3f::new(0.0, 0.0, 0.0);
        let mut actual_res: Option<RaycastResult> = None;
        for (i, geom) in self.geometry.iter().enumerate() {
            let res = geom.raycast(ray);
            if let Some(hit) = res {
                if hit.distance < prev_dist {
                    let material = self.get_material(i).unwrap();
                    prev_dist = hit.distance;
                    color = shade(self, &hit, &material);
                    actual_res = res;
                }
            }
        }

        // Recurse
        if max_iters > 0 {
            if let Some(hit) = actual_res {
                let new_dir = (ray.origin - hit.position)
                    .normalized()
                    .reflected(&hit.normal);
                let new_ray = Ray::new(hit.position, new_dir);
                color = color + self.trace(&new_ray, max_iters - 1) * 0.95;
            }
        }

        color
    }

    pub fn get_material(&self, id: usize) -> Option<Material> {
        for (name, indices) in self.material_map.iter() {
            if indices.contains(&id) {
                return self.materials.get(name).cloned();
            }
        }
        None
    }
}

fn shade(scene: &Scene, res: &RaycastResult, material: &Material) -> Vec3f {

    let v = (scene.camera.position - res.position).normalized();
    let mut color = Vec3f::new(0.0, 0.0, 0.0);
    for light in scene.lights.iter() {
        
            let l = light.light_vector(&res.position);
            let h = (l + v).normalized();
        
            let ambient = 0.1;
            let lambertian = res.normal.dot(&l).max(0.0);
            let specular = res.normal.dot(&h).max(0.0);
            let specular = specular.powi(30);
            println!("{:?}", light.color());
            color = color + material.color * (ambient + lambertian) + specular * light.color()

    }
    color
}
