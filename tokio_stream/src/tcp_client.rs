use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let server_address = "127.0.0.1:10000";

    // Connect to the server
    let mut stream = TcpStream::connect(server_address).await.unwrap();
    println!("Connected to server at {}", server_address);

    // Send data to the server
    let data = "Hello, server!";
    if let Err(e) = stream.write_all(data.as_bytes()).await {
        eprintln!("Failed to send data to server: {}", e);
        return;
    }
    println!("Sent data to server: {}", data);

    /*
    Receive response from the server
        let mut buffer = [0; 1024];
        let n = match stream.read(&mut buffer).await {
            Ok(n) => n,
            Err(e) => {
                eprintln!("Failed to receive response from server: {}", e);
                return;
            }
        };
        let response = String::from_utf8_lossy(&buffer[..n]);
    println!("Received response from server: {}", response);
    */
    // Close the connection
    drop(stream);
    println!("Connection closed");
}
