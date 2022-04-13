/*
 * @Author: Lmmqxyx
 * @Date: 2022-04-13 17:57:15
 * @LastEditors: Please set LastEditors
 * @LastEditTime: 2022-04-13 18:07:26
 * @FilePath: \Rust_Tutorial\src\simple\ch_6_constants.rs
 * @Description:
 */
static LANGUAGE: &str = "RUSt";
const YEAR: u32 = 2022;
#[test]
fn main() {
    // let word = "hello world";
    println!("hello world {}", YEAR);
    println!("language is {}", LANGUAGE);
}
