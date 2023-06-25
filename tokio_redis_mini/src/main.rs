use std::collections::HashMap;

use mini_redis::{
    Command::{self, Get, Set},
    Connection, Frame,
};
use tokio::net::{TcpListener, TcpStream};


#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    loop {
        let (socket, socket_addr) = listener.accept().await.unwrap();
        tokio::spawn(async {
            process(socket).await;
        });
    }
}

async fn process(socket: TcpStream) {
    let mut connection = Connection::new(socket);
    let mut db = HashMap::new();

    while let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT :{:?}", frame);

        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("Ok, set a value successfully".to_string())
            }
            Get(cmd) => {
                if let Some(val) = db.get(cmd.key()) {
                    Frame::Bulk(val.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}
