/*
 * @Author: Lmmqxyx
 * @Date: 2022-05-02 23:54:25
 * @LastEditors: Please set LastEditors
 * @LastEditTime: 2022-05-03 23:59:38
 * @FilePath: \Rust_Tutorial\src\simple\ch_10_string.rs
 * @Description: 
 */
#[test]
fn hello() {
    let p: &'static str="Hello world madam!";
    println!("{}",p);
    for word in p.split_whitespace().rev() {
        println!("> {}",word);
    }
    let mut chars=p.chars().collect::<Vec<char>>();
    chars.sort();
    chars.dedup();
    let mut string =String::new();
    for c in chars {
        string.push(c);
        string.push_str(", ");
        println!("{}",c);
    }
    println!("{}",string);
}
