/*
 * @Author: Lmmqxyx
 * @Date: 2022-04-07 18:07:57
 * @LastEditors: Please set LastEditors
 * @LastEditTime: 2022-05-02 23:56:44
 * @FilePath: \Rust_Tutorial\src\simple\ch_3_convert.rs
 * @Description: 
 */
fn char_to_u8(a: char) -> u8 {
    a as u8
}

fn i32_to_char(a:i32)->char {
    a as u8 as char
}

fn string_to_char_array(_s:String)->Vec<char>{
    vec!['a']
}
#[test]
fn var() {
    let mut a = 'a';
    // println!("{}",a as u8);
    assert_eq!(97,char_to_u8(a));
    assert_eq!(a,i32_to_char(97));
}
