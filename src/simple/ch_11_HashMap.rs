/*
 * @Author: Lmmqxyx
 * @Date: 2022-05-04 00:00:32
 * @LastEditors: Please set LastEditors
 * @LastEditTime: 2022-05-16 23:18:38
 * @FilePath: \Rust_Tutorial\src\simple\ch_11_hashmap.rs
 * @Description: 
 */
use std::collections::HashMap;
#[test]
fn hash_map() {
    let mut contacts=HashMap::new();
    contacts.insert("Bob","1336574");
    contacts.insert("Alice","123456");
    // Takes a reference and returns Option<&V>
    match contacts.get(&"Bob"){
        Some(&number) =>{
            println!("{}",number);
            assert_eq!("1336574",number);
        },
        _ =>{
            println!("Do not have Bob's number")
        },
    }
    contacts.remove(&"Alice");
    for (contact,number) in contacts.iter() {
        println!("Calling {}: {}",contact,number);
    }
}
