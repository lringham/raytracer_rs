use tonic::{transport::Server, Request, Response, Status};

use raytracer::file_dispatcher_server::{FileDispatcher, FileDispatcherServer};
use raytracer::{FilePathReply, FilePathRequest};

pub mod raytracer {
    tonic::include_proto!("raytracer");
}

#[derive(Debug, Default)]
pub struct MyFileDispatcher {}

#[tonic::async_trait]
impl FileDispatcher for MyFileDispatcher {
    async fn get_file_path(
        &self,
        request: Request<FilePathRequest>,
    ) -> Result<Response<FilePathReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = FilePathReply {
            filepath: format!("assets/scene.json"),
        };

        Ok(Response::new(reply))
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
