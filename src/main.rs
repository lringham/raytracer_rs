mod camera;
mod geometry;
mod material;
mod ppm;
mod raycast;
mod scene;
mod vec3f;
mod light;

use std::env;

use rayon::prelude::*;
use scene::Scene;
use vec3f::Vec3f;

fn get_scene_path() -> Option<String> {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        Some(args[1].clone())
    } else {
        None
    }
}

fn main() {
    // Load scene
    let scene_path = get_scene_path().expect("Scene.json path not provided!");
    let scene = Scene::from(&scene_path).expect("Failed to load scene!");
    let (width, height) = scene.camera.resolution;

    // Trace the scene
    let mut data = vec![Vec3f::new(0.0, 0.0, 0.0); width * height];
    let chunk_size = num_cpus::get();
    data.par_chunks_mut(chunk_size)
        .enumerate()
        .for_each(|(chunk_idx, chunk)| {
            let start_idx = chunk_idx * chunk_size;
            for (offset, value) in chunk.iter_mut().enumerate() {
                let i = start_idx + offset;
                let x = i % width;
                let y = i / height;
                let ray = scene.camera.get_ray(x, y);
                *value = scene.trace(&ray, 10);
            }
        });

    // Save output
    let framebuffer = ppm::Ppm::from(width, height, &data);
    if let Err(e) = framebuffer.save("image.ppm") {
        eprintln!("Error saving image: {}", e);
    }
}
