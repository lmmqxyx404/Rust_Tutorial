fn char_to_u8(a: char) -> u8 {
    a as u8
}

fn i32_to_char(a:i32)->char {
    a as u8 as char
}
#[test]
fn var() {
    let mut a = 'a';
    // println!("{}",a as u8);
    assert_eq!(97,char_to_u8(a));
    assert_eq!(a,i32_to_char(97));
}
