use std::io::Result;


#[tokio::main]
async fn main() -> Result<()> {
    let body = reqwest::get("https://www.miercn.com").await.unwrap();

    // println!("body = {:?}", body);
    Ok(())
}
