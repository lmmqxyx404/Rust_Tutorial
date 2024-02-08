use warp::Filter;
use std::env;

#[tokio::main]
async fn main() {
    // Get the directory path from the command line arguments
    let args: Vec<String> = env::args().collect();
    let mut dir;
    if args.len() < 2 {
        eprintln!("Usage: {} <directory_to_serve>", args[0]);
        dir = "/home".to_string();
        // std::process::exit(1);
    }else {
        dir = args[1].clone();
    }

    println!("Serving files from: {}", dir);
    // Specify the directory you want to serve files from

    // Create a filter for the files
    let files = warp::fs::dir(dir);

    // Start the server
    warp::serve(files)
        .run(([127, 0, 0, 1], 3030))
        .await;
}