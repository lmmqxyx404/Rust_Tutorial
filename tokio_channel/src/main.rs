use chrono::Local;
use std::{thread, time::Duration};
use tokio::{self, runtime::Runtime, time};

fn now() -> String {
    Local::now().format("%F %T").to_string()
}

async fn say(i: u64) {
    println!("task: {} wait before {}", i, now());
    tokio::time::sleep(Duration::from_secs(i)).await;
    println!("task: {} wait after {}", i, now());
}

// The synchronous task will be executed in priority order.
// Only meet the await keyword, then the task will be 
async fn execute() {
    let mut tasks = Vec::new();
    for i in 3..8 {
        tasks.push(tokio::spawn(async move {
            say(i).await;
        }));
    }
    println!("task: main wait before {}", now());
    println!("All tasks have completed.");
    let mut counter = 3;
    for i in tasks {
        counter += 1;
        println!("task: loop wait before {}", now());
        if counter == 6 {
            i.await;
            break;
        }
    }
    println!("All tasks have completed. again");
}
#[tokio::main]
async fn main() {
    execute().await;
    // let (tx,rx)=
    // Execute all tasks concurrently and wait for them to complete
    /* while let Some(task) = tasks.pop() {
        // task.await.unwrap();
        println!(" {}", now());
    } */
    // Print the results of each task
    /* for result in results {
        println!("Task result: {}", result);
    } */
}
