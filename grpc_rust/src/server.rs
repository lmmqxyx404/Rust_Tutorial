use records::recorder_server::Recorder;
// use records::{Recode}

pub mod records {
    tonic::include_proto!("records");
}

#[derive(Debug, Default)]
pub struct RecorderService {}

#[tonic::async_trait]
impl Recorder for RecorderService {
    async fn send_message(&self, msg: _) {
        
    }
}

fn main() {
    println!("Starting server...");
}
