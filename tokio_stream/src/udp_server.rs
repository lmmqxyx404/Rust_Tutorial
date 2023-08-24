use tokio::net::UdpSocket;

#[tokio::main]
async fn main() {
    // Bind the UDP socket to a specific address
    let addr = "127.0.0.1:11000";
    let socket = UdpSocket::bind(addr).await.unwrap();
    println!("UDP server listening on {}", addr);

    // Create a buffer to store received data
    let mut buf = [0u8; 1024];

    loop {
        // Receive a UDP datagram
        let (size, peer) = match socket.recv_from(&mut buf).await {
            Ok((size, peer)) => (size, peer),
            Err(e) => {
                eprintln!("Failed to receive data: {}", e);
                continue;
            }
        };

        // Process the received data
        let data = &buf[..size];
        let message = String::from_utf8_lossy(data);
        println!("Received message from {}: {}", peer, message);

        // Send a response back to the client
        let response = "Hello, client!";
        if let Err(e) = socket.send_to(response.as_bytes(), &peer).await {
            eprintln!("Failed to send response: {}", e);
        }
    }
}