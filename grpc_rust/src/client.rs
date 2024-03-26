use tokio::time::{sleep, Duration};

use hero::heroes_service_client::HeroesServiceClient;
use hero::Hero;

use records::recorder_client::RecorderClient;
use records::RecordRequest;
use tonic::Request;

pub mod records {
    tonic::include_proto!("records");
}

pub mod hero {
    tonic::include_proto!("hero");
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting client...");
    let mut client = RecorderClient::connect("http://127.0.0.1:60000").await?;
    // let mut client2 = HeroesServiceClient::connect("http://127.0.0.1:60000").await?;
    // let mut client2=
    let mut counter = 1;
    loop {
        // counter++;
        println!("{}", counter);
        let request = Request::new(RecordRequest {
            user_name: "hello lmmqxyx".to_string(),
            user_age: 26,
        });
        let resp = client.say_hello(request).await?;
        println!("{:#?}", resp);
        let resp = client.say_secret(()).await?;
        println!("{:#?}", resp);
        counter += 1;
        sleep(Duration::from_secs(5)).await;
    }

    Ok(())
}
