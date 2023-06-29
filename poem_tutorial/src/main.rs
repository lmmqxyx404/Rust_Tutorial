use poem::{get, handler, listener::TcpListener, web::Path, IntoResponse, Route, Server};

#[handler]
fn hello(Path(name): Path<String>) -> String {
    format!("hello: {}", name)
}

async fn build_a_server() -> Result<(), std::io::Error> {
    let app = Route::new().at("/hello/:name", get(hello));
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let fu = build_a_server();
    
    fu.await
}
