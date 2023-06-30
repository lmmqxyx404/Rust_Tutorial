use tokio::net::{TcpListener, TcpStream};
use tokio::io::{copy, BufReader, BufWriter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:9000").await?;
    println!("Listening on {}", "127.0.0.1:9000");

    loop {
        let (inbound, _) = listener.accept().await?;

        let outbound = TcpStream::connect("127.0.0.1:8899").await?;

        tokio::spawn(async move {
            let (mut ri, mut wi) = inbound.into_split();
            let (mut ro, mut wo) = outbound.into_split();

            let server_to_client = tokio::spawn(async move {
                let mut reader = BufReader::new(&mut ro);
                let mut writer = BufWriter::new(&mut wi);
                copy(&mut reader, &mut writer).await.unwrap();
            });

            let client_to_server = tokio::spawn(async move {
                let mut reader = BufReader::new(&mut ri);
                let mut writer = BufWriter::new(&mut wo);
                copy(&mut reader, &mut writer).await.unwrap();
            });

            tokio::join!(server_to_client, client_to_server);
        });
    }
}