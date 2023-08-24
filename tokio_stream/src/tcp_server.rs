use tokio::io::{copy, AsyncReadExt, BufReader, BufWriter};
use tokio::net::{TcpListener, TcpStream};

// transport the data flow
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:10000").await?;
    println!("Listening on {}", "127.0.0.1:10000");

    loop {
        let (mut inbound, _) = listener.accept().await?;

        // let outbound = TcpStream::connect("127.0.0.1:8899").await?;

        tokio::spawn(async move {
            let mut buffer = [0; 1024];
            let n = match inbound.read(&mut buffer).await {
                Ok(n) => {
                    if n == 0 {
                        println!("can not get more contents");
                        return;
                    } else {
                        n
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read from socket: {}", e);
                    return;
                }
            };
            println!("{:?}", buffer);
        });
    }
}
