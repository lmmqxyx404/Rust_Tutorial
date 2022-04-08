/*
 * @Author: Lmmqxyx
 * @Date: 2022-04-07 18:13:26
 * @LastEditors: Please set LastEditors
 * @LastEditTime: 2022-04-08 18:13:47
 * @FilePath: \Rust_Tutorial\src\simple\ch_4_struct.rs
 * @Description:
 * struct initialize must bu named the key. also you can name a same name variable
 */
struct Point {
    x: f32,
    y: f32,
}

#[test]
fn valid() {
    let p = Point { x: 0.1, y: 0.5 };
    assert_eq!(1u8, 1u8);
}
