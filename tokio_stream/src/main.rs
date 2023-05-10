use std::io::{Error, ErrorKind, Result};

use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<()> {
    let stream = TcpStream::connect("www.baidu.com:443".to_string()).await;
    let stream = match stream {
        Ok(stream) => {
            println!("success");
            stream
        }
        Err(e) => {
            println!("err {:?}", e.to_string());
            return Err(Error::new(ErrorKind::ConnectionReset, e));
        }
    };
    Ok(())
}
