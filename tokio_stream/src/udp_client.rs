use tokio::net::UdpSocket;

#[tokio::main]
async fn main() {
    let server_addr = "127.0.0.1:11000";
    let socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();
    socket.connect(server_addr).await.unwrap();
    println!("Connected to UDP server at {}", server_addr);

    let message = "Hello, server!";
    let message_bytes = message.as_bytes();

    // Send a message to the server
    if let Err(e) = socket.send(message_bytes).await {
        eprintln!("Failed to send data: {}", e);
        return;
    }
    println!("Sent message to server: {}", message);

    // Receive a response from the server
    let mut buf = [0u8; 1024];
    if let Ok((size, _)) = socket.recv_from(&mut buf).await {
        let response = String::from_utf8_lossy(&buf[..size]);
        println!("Received response from server: {}", response);
    } else {
        eprintln!("Failed to receive response from server");
    }
}