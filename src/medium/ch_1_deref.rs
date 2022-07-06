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

    // copy a reference
    let a = 45i32;
    let b = &a;
    let c = b;
    assert_eq!(*c, 45);
    let d = &b;
    // assert_eq!(*d, 45);
    assert_eq!(**d, 45);
    /// because the  original variable is not mutable.So it can not
    /// be referred as mutable.
    /// let asd = &mut a;
    let mut a = 45;
    let asd = &mut a;
    *asd = *asd + 60;
    assert_eq!(*asd, 105);
}
