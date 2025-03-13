use std::{fs::File, io::BufReader};

use crate::{
    camera::Camera,
    geometry::Geometry,
    raycast::{Ray, RaycastResult, Raycastable},
    vec3f::Vec3f,
};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Scene {
    pub bg_color: Vec3f,
    pub light_pos: Vec3f,
    pub light_col: Vec3f,
    pub material_col: Vec3f,
    pub camera: Camera,
    pub geometry: Vec<Geometry>,
}

impl Scene {
    pub fn from(filename: &str) -> Option<Scene> {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let scene: Scene = serde_json::from_reader(reader).unwrap();
        Some(scene)
    }

    pub fn trace(&self, ray: &Ray, max_iters: usize) -> Vec3f {        
        let mut prev_dist = f32::MAX;
        let mut color = self.bg_color;
        
        let mut actual_res : Option<RaycastResult> = None;
        for geom in self.geometry.iter() {
            let res = geom.raycast(ray);
            if let Some(hit) = res {
                if hit.distance < prev_dist {
                    prev_dist = hit.distance;
                    color = shade(self, &hit);
                    actual_res = res;
                }
            }
        }

        if  max_iters > 0 {
            if let Some(hit) = actual_res{
                let new_dir = (ray.origin - hit.position).normalized().reflected(&hit.normal);
                let new_ray = Ray::new(hit.position, new_dir);
                color = color + self.trace(&new_ray, max_iters - 1);
            }
        }
        
        color
    }
}

fn shade(scene: &Scene, res: &RaycastResult) -> Vec3f {
    let l = (scene.light_pos - res.position).normalized();
    let v = (scene.camera.position - res.position).normalized();
    let h = (l + v).normalized();

    let ambient = 0.1;
    let lambertian = res.normal.dot(&l).max(0.0);
    let specular = res.normal.dot(&h).max(0.0);
    let specular = specular.powi(30);

    scene.material_col * (ambient + lambertian) + specular * scene.light_col
}
