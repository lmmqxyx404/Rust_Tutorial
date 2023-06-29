use std::{collections::HashMap, fmt::format, time::Duration};

/*
In this example, the `producer()` task sends integers to the `consumer()` task through an `mpsc` channel. The `channel()` function creates an `mpsc::Sender` and an `mpsc::Receiver` with a buffer size of 10. The `producer_task` and `consumer_task` are spawned using `tokio::spawn()`, and their handles are awaited at the end of the `main()` function.
*/
use actix_web::{get, post, rt, web, App, HttpResponse, HttpServer, Responder};
use tokio::sync::mpsc::{self, Sender};
// Define a message type to send between tasks
#[derive(Debug)]
struct Message {
    relative: String,
}

#[get("/")]
async fn hello(
    tx: web::Data<Sender<Message>>,
    query: web::Query<HashMap<String, String>>,
) -> impl Responder {
    let mut msg = Message {
        relative: "aaa".to_string(),
    };
    if let Some(str) = query.get("name") {
        msg.relative = str.clone();
    }
    let p = msg.relative.clone();
    tx.send(msg).await.unwrap();
    HttpResponse::Ok().body(format!("hello {}", p))
}

// Define a handler function for the GET request
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

// #[actix_web::main]
async fn build_server() {
    // Create an mpsc channel to communicate between the server and the worker task
    // let (tx, rx) = mpsc::channel(10);

    // Spawn the worker task
    // tokio::spawn(worker(rx));2

    // Start the HTTP server
    HttpServer::new(move || {
        App::new().service(echo) // Register the handler function for the GET request
                                 // .data(tx.clone())  // Pass a clone of the sender to the handler function
    })
    .bind("127.0.0.1:8080")
    .unwrap() // Bind the server to localhost on port 8080
    .run()
    .await
    .unwrap()
}

async fn producer(tx: mpsc::Sender<i32>) {
    /* let server = HttpServer::new(|| {
        App::new()
            // .service(hello)
            // .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))
    .unwrap();
    match server.run().await {
        Ok(_) => {
            println!("build a server");
        }
        Err(e) => {
            panic!("can not build a server");
        }
    }; */
    for i in 0..10 {
        println!("send {}", i);
        tx.send(i).await.unwrap();
        tokio::time::sleep(Duration::from_millis((i * 23 * 23) as u64 % 237)).await;
    }
    /*   */
}

async fn consumer(mut rx: mpsc::Receiver<i32>) {
    while let Some(i) = rx.recv().await {
        println!("Received {}", i);
    }
}

async fn worker(mut rx: mpsc::Receiver<Message>) {
    while let Some(message) = rx.recv().await {
        // Do some work with the received message
        println!("Received message: {:?}", message);
    }
}
/*
// Define a handler function for the GET request
#[get("/{relative}")]
async fn index(
    web::Path(relative): web::Path<String>,
    tx: web::Data<mpsc::Sender<Message>>,
) -> impl Responder {
    // Send the parsed parameter to another task using the mpsc channel
    let message = Message { relative };
    tx.send(message).await.unwrap();

    // Respond with a message indicating that the message has been sent
    HttpResponse::Ok().body("Message sent")
}
*/

async fn build_a_server() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind("127.0.0.1:9999")?
        .run()
        .await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create an mpsc channel to communicate between the server and the worker task
    let (tx, rx) = mpsc::channel(10);

    // Spawn the worker task
    tokio::spawn(worker(rx));

    // Start the HTTP server
    rt::spawn(async move {
        HttpServer::new(move || {
            App::new()
                .service(hello) // Register the handler function for the GET request
                .app_data(web::Data::new(tx.clone())) // Pass a clone of the sender to the handler function
        })
        .bind("127.0.0.1:8080")
        .unwrap() // Bind the server to localhost on port 8080
        .run()
        .await
    });

    loop {}
    /* let producer_task = tokio::spawn(producer(tx));
    let consumer_task = tokio::spawn(consumer(rx));

    producer_task.await.unwrap();
    consumer_task.await.unwrap(); */
}
