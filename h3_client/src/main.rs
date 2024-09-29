fn main() {
    let req = http::Request::post("https://cdn.lmmqxyx.us.kg:443/auth")
        .header("Hysteria-Auth", "asdfghjkl")
        .header("Hysteria-CC-RX", "0")
        .header("Hysteria-Padding", "[25,123,58,96,321,456,789]")
        .body(())
        .unwrap();
    println!("Hello, world!");
}
