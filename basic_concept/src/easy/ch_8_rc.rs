/*
 * @Author: Lmmqxyx
 * @Date: 2022-04-25 23:46:20
 * @LastEditors: &Please set LastEditors
 * @LastEditTime: 2022-04-25 23:53:21
 * @FilePath: \Rust_Tutorial\src\simple\ch_8_rc.rs
 * @Description:
 */
use std::{borrow::Borrow, cell::RefCell, collections::btree_map::Values, rc::Rc};
// the realtion between & and reference
fn reference_rc() {
    let a = 45;
    let b = &a;
    let c = &a;
    assert_eq!(*b, *c);
    let d = Rc::new(45);
    assert_eq!(*d, a);
}
#[test]
fn hello() {
    reference_rc();
    let word = "hello world".to_string();
    {
        println!("create a rc instance");
        let rc_a = Rc::new(word);
        println!("Value of rc_a is {}", rc_a);
        println!("Reference counter of rc_a is {}", Rc::strong_count(&rc_a));
        {
            println!("create another rc instance");
            let rc_b = Rc::clone(&rc_a);
            println!("Reference counter of rc_a is {}", Rc::strong_count(&rc_a));
            println!("Reference counter of rc_b is {}", Rc::strong_count(&rc_b));
            assert_eq!(rc_a, rc_b);
            println!("Value of rc_b is {}", rc_b);
            println!("rc_b is dropped out of scope ");
        }
        println!(
            "Now reference counter of rc_a is {}",
            Rc::strong_count(&rc_a)
        );
        println!("rc_a is dropped out of scope ");
    }
    // assert_eq!("hello world", word);
    let a = 5;
    let c = &a;
    assert_eq!(5, *c);
    let b = RefCell::new(a);
    *b.borrow_mut() += 14;
    let d = b.into_inner();
    assert_eq!(d, 19);
    assert_eq!(a, 5);
    let b = RefCell::new(14);
    *b.borrow_mut() += 14;
    assert_eq!(b.into_inner(), 28);
    // pay attention to the 58 line

    let c = Rc::new(RefCell::new(45));
    *c.borrow_mut() = 2;
    assert_eq!((*c).take(), 2);
}
