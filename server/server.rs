use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{transport::Server, Request, Response, Status};

use raytracer::file_dispatcher_server::{FileDispatcher, FileDispatcherServer};
use raytracer::{FilePathReply, FilePathRequest, ImageData, ImageDataReply, ImageDataRequest};

pub mod raytracer {
    tonic::include_proto!("raytracer");
}

#[derive(Debug, Default)]
pub struct MyFileDispatcher {
    saved_data: Arc<Mutex<Option<ImageDataReply>>>,
}

#[tonic::async_trait]
impl FileDispatcher for MyFileDispatcher {
    async fn get_file_path(
        &self,
        request: Request<FilePathRequest>,
    ) -> Result<Response<FilePathReply>, Status> {
        println!("Got a request: {:?}", request);
        let reply = FilePathReply {
            filepath: "assets/scene.json".to_string(),
        };
        Ok(Response::new(reply))
    }

    async fn get_image_data(
        &self,
        _request: Request<ImageDataRequest>,
    ) -> Result<Response<ImageDataReply>, Status> {
        println!("Got a request for image data");

        { // If saved image data exists, return that
            let saved_data = self.saved_data.lock().await;
            if let Some(saved) = &*saved_data {
                return Ok(Response::new(saved.clone()));
            }
        }

        // Default image
        let mut image_data = vec![
            ImageData {
                r: 1.0,
                g: 0.0,
                b: 0.0
            };
            100 * 100
        ];
        for y in 0..100 {
            for x in 0..100 {
                image_data[x + y * 100].r = x as f32 / 99.0;
                image_data[x + y * 100].g = y as f32 / 99.0;
            }
        }
        let reply = ImageDataReply {
            width: 100,
            height: 100,
            data: image_data,
        };
        Ok(Response::new(reply))
    }

    async fn update_image_data(
        &self,
        request: Request<ImageDataReply>,
    ) -> Result<Response<FilePathReply>, Status> {
        println!("Updating image data...");

        let mut saved_data = self.saved_data.lock().await;
        *saved_data = Some(request.into_inner());

        Ok(Response::new(FilePathReply {
            filepath: "updated".to_string(),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let file_dispatcher = MyFileDispatcher::default();
    Server::builder()
        .add_service(FileDispatcherServer::new(file_dispatcher))
        .serve(addr)
        .await?;
    Ok(())
}
