use minifb::{Window, WindowOptions};
use raytracer::file_dispatcher_client::FileDispatcherClient;
use raytracer::{ImageData, ImageDataRequest};

use std::{thread, time};
pub mod raytracer {
    tonic::include_proto!("raytracer");
}

fn to_color(datum: &ImageData) -> u32 {
    u32::from_be_bytes([
        255_u8,
        (datum.r * 255.0) as u8,
        (datum.g * 255.0) as u8,
        (datum.b * 255.0) as u8,
    ])
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        println!("Requesting scene data...");
        let mut client = FileDispatcherClient::connect("http://[::1]:50051").await?;
        let request = tonic::Request::new(ImageDataRequest {});
        let response = client.get_image_data(request).await?;
        let data = response.get_ref();

        //
        let width = data.width as usize;
        let height = data.height as usize;
        let mut buffer: Vec<u32> = vec![0; (width * height) as usize];
        for (i, datum) in data.data.iter().rev().enumerate() {
            let color = to_color(datum);
            buffer[i] = color;
        }

        // Create window
        let mut window = Window::new("rt_viewer", width, height, WindowOptions::default())
            .unwrap_or_else(|e| {
                panic!("{}", e);
            });

        window.update_with_buffer(&buffer, width, height).unwrap();

        let one_sec = time::Duration::from_secs(5);
        thread::sleep(one_sec);
    }
}
