use std::io::{Error, ErrorKind};

use mini_redis::{client, Result};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<()> {
    // connect to redis server
    let mut client = client::connect("127.0.0.1:6379").await?;
    // set key
    client.set("hello", "value is world".into()).await?;
    // get key
    let res = client.get("hello").await?;
    println!("{:?}", res);
    Ok(())
}
