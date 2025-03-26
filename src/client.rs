mod camera;
mod geometry;
mod light;
mod material;
mod ppm;
mod raycast;
mod scene;
mod vec3f;

use rayon::prelude::*;
use scene::Scene;
use vec3f::Vec3f;

use raytracer::file_dispatcher_client::FileDispatcherClient;
use raytracer::{FilePathRequest, ImageData, ImageDataReply};
use simple_tqdm::ParTqdm;
pub mod raytracer {
    tonic::include_proto!("raytracer");
}

fn render(scene: &Scene) -> Vec<Vec3f> {
    println!("Tracing...");
    let (width, height) = scene.camera.resolution;
    let mut data = vec![Vec3f::new(0.0, 0.0, 0.0); width * height];
    let chunk_size = num_cpus::get();
    data.par_chunks_mut(chunk_size)
        .enumerate()
        .tqdm()
        .for_each(|(chunk_idx, chunk)| {
            let start_idx = chunk_idx * chunk_size;
            for (offset, value) in chunk.iter_mut().enumerate() {
                let i = start_idx + offset;
                let x = i % width;
                let y = i / height;
                let ray = scene.camera.get_ray(x, y);
                *value = scene.render(&ray, 10);
            }
        });
    data
}

// fn save(out_path: &str, data: &[Vec3f], dimensions: (usize, usize)) {
//     let (width, height) = dimensions;
//     println!("Saving image: {}", out_path);
//     let framebuffer = ppm::Ppm::from(width, height, data);
//     if let Err(e) = framebuffer.save(out_path) {
//         eprintln!("Error saving image: {}", e);
//     }
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Requesting scene file...");
    let mut client = FileDispatcherClient::connect("http://[::1]:50051").await?;
    let request = tonic::Request::new(FilePathRequest {});
    let response = client.get_file_path(request).await?;
    let scene_path = response.get_ref().filepath.clone();
    let scene = Scene::from(&scene_path).expect("Failed to load scene!");

    let data = render(&scene);

    let (width, height) = scene.camera.resolution;
    let mut image_data = vec![
        ImageData {
            r: 0.0,
            g: 0.0,
            b: 0.0
        };
        width * height
    ];
    for (i, datum) in image_data.iter_mut().enumerate() {
        datum.r = data[i].x;
        datum.g = data[i].y;
        datum.b = data[i].z;
    }

    // let imageData = ImageData;
    let request = tonic::Request::new(ImageDataReply {
        width: width as u64,
        height: height as u64,
        data: image_data,
    });
    client.update_image_data(request).await?;

    Ok(())
}
