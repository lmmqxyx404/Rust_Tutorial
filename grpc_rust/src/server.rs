use records::recorder_server::Recorder;
use records::{RecordRequest, RecordResponse};
use tonic::transport::Server;
use tonic::{Request, Response, Status};

use crate::records::recorder_server::RecorderServer;
// use records::{Recode}

pub mod records {
    tonic::include_proto!("records");
}

#[derive(Debug, Default)]
pub struct RecorderService {}

#[tonic::async_trait]
impl Recorder for RecorderService {
    async fn say_hello(
        &self,
        request: Request<RecordRequest>,
    ) -> Result<Response<RecordResponse>, Status> {
        println!("request: {:#?}", request);
        let req = request.into_inner();
        let response = RecordResponse {
            successful: true,
            message: format!("User {} is {} old", req.user_name, req.user_age).into(),
        };
        Ok(Response::new(response))
    }

    async fn say_secret(&self, req: Request<()>) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket_addr = "127.0.0.1:60000".parse()?;

    let record_service = RecorderService::default();

    Server::builder()
        .add_service(RecorderServer::new(record_service))
        .serve(socket_addr)
        .await?;
    println!("Starting server...");
    Ok(())
}
