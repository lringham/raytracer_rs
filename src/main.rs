mod geometry;
mod ppm;
mod raycast;
mod scene;
mod vec3f;

use std::env;

use raycast::{Ray, Raycastable};
use scene::Scene;
use vec3f::Vec3f;

fn main() {
    // Load scene
    let mut scene_path = "scene.json";
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        scene_path = &args[1];        
    }
    let scene = Scene::from(scene_path).expect("Failed to load scene");

    // Setup framebuffer
    let px_size = 0.01;
    let width = 500;
    let height = 500;
    let mut framebuffer = ppm::Ppm::new(width, height);

    // Main loop
    for y in 0..width {
        for x in 0..height {

            // Create a ray 
            let dir = Vec3f::new(
                px_size * (x - width / 2) as f32,
                px_size * (y - height / 2) as f32,
                -1.0,
            )
            .normalized();
            let ray = Ray::new(scene.camera_pos, dir);

            // Raycast scene
            let mut color = scene.bg_color;
            for geom in scene.geometry.iter() {
                let hit = geom.raycast(&ray);                
                if let Some(res) = hit {
                    let l = (scene.light_pos - res.hit).normalized();
                    let v = (scene.camera_pos - res.hit).normalized();                    
                    let h = (l + v).normalized();

                    // ambient
                    let ambient = 0.1;

                    // diffuse
                    let lambertian = res.normal.dot(&l).max(0.0);
                    
                    // specular 
                    let specular = res.normal.dot(&h).max(0.0); 
                    let specular = specular.powi(30);
                    
                    // Blinn Phong
                    color = scene.material_col * (ambient + lambertian) + specular * scene.light_col;
                }

                // Write to FB
                framebuffer.set(x, height - 1 - y, color);
            }
        }
    }

    // Save output
    if let Err(e) = framebuffer.save("image.ppm") {
        eprintln!("Error saving image: {}", e);
    }
}
