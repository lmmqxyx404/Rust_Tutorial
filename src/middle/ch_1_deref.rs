/*
 * @Author: Lmmqxyx
 * @Date: 2022-06-21 07:42:10
 * @LastEditors: Please set LastEditors
 * @LastEditTime: 2022-06-28 23:55:11
 * @FilePath: \Rust_Tutorial\src\middle\ch_1_deref.rs
 * @Description:
 */
use std::ops::Deref;
struct My_Box<T>(T);

impl<T> My_Box<T> {
    fn new(x: T) -> My_Box<T> {
        My_Box(x)
    }
}

impl<T> Deref for My_Box<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
#[test]
fn middle_1_deref() {
    // pay aattention to the mutable reference
    let c = "ccccc";
    let mut calias = &c;
    calias = &"ddd";
    println!("This is middle {}", *calias);
    // imply the trait deref for T so that T can be dereferenced.
    let a = 5;
    let b = Box::new(a);
    let c = My_Box::new(a);
    assert_eq!(a, *b);
    assert_eq!(a, *c);
}
