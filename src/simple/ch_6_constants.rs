/*
 * @Author: Lmmqxyx
 * @Date: 2022-04-13 17:57:15
 * @LastEditors: Please set LastEditors
 * @LastEditTime: 2022-04-23 23:29:13
 * @FilePath: \Rust_Tutorial\src\simple\ch_6_constants.rs
 * @Description:
 */
static LANGUAGE: &str = "rustc-std-workspace-alloc";
const YEAR: u32 = 2022;
#[test]
fn main() {
    // let word = "hello world";
    println!("hello world {}", YEAR);
    println!("language is {}", LANGUAGE);
    assert_eq!("RUST",LANGUAGE);
}
