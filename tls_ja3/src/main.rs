use ja3::Ja3;

fn main() {
    // chrome83
    let mut ja3 = Ja3::new("/home/si/Desktop/Rust_Linux/Rust_Tutorial/tls_ja3/asdf.pcap")
        .process_pcap()
        .unwrap();

    let mut p = ja3.into_iter();
    while let Some(hash) = p.next() {
        println!("Hash: {}", hash);
        println!("Hash: {:?}", hash.hash);

        println!("Hash: {:?}", hash.ja3_str);
    }
}

/// 将字符串转为数字
fn string_to_num(str: String) -> Vec<i32> {
    // let str = "0-23-65281-10-11-35-16-5-13-18-51-45-43-27-21".to_string();
    let vv: Vec<&str> = str.split('-').collect();
    let nums: Vec<Result<i32, _>> = vv.iter().map(|x| x.parse::<i32>()).collect();
    return nums.iter().map(|x| x.clone().unwrap_or(-1)).collect();
}

#[cfg(test)]
mod tests {
    use crate::string_to_num;

    #[test]
    fn test_for_ja3() {
        let nums = string_to_num("0-23-65281-10-11-35-16-5-13-18-51-45-43-27-21".to_string());
        println!("{:?}", nums);
    }
}
