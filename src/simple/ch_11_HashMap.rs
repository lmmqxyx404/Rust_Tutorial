/*
 * @Author: Lmmqxyx
 * @Date: 2022-05-04 00:00:32
 * @LastEditors: Please set LastEditors
 * @LastEditTime: 2022-05-05 14:07:26
 * @FilePath: \Rust_Tutorial\src\simple\ch_11_HashMap.rs
 * @Description:
 */
use std::collections::HashMap;
#[test]
fn main() {
    let mut contacts = HashMap::new();
    contacts.insert("Bob", "1336574");
    contacts.insert("Alice", "12345678");
    match contacts.get(&"Bob") {
        Some(number) => {
            println!("Bob's number is {:?}", number);
        }
        _ => {
            println!("There is no Bob's number!");
        }
    }
    contacts.remove(&"Bob");
    for (&a, b) in contacts.iter() {
        println!("{}'s number is {}", &a, &b);
    }
}
