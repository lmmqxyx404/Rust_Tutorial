use ja3::Ja3;

fn main() {
    
    let mut ja3 = Ja3::new("/home/si/Desktop/Rust_Linux/Rust_Tutorial/tls_ja3/aaa.pcap").process_pcap().unwrap();
    let mut p=ja3.into_iter();
    while let Some(hash) = p.next() {
        println!("Hash: {}", hash);
    }
}
