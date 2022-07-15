/*
 * @Author: Lmmqxyx
 * @Date: 2022-04-07 18:07:48
 * @LastEditors: Please set LastEditors
 * @LastEditTime: 2022-07-07 22:48:27
 * @FilePath: \Rust_Tutorial\src\hard\ch_0_hello.rs
 * @Description:
 */
#[test]
fn ch_2_vec_iter() {
    let a = vec![String::from("hello"), String::from("world")];
    let sum = &a.iter().fold(String::from(""), |acc, x| acc + " " + x);
    assert_eq!(sum, " hello world")
}
