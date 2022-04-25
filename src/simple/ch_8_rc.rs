/*
 * @Author: Lmmqxyx
 * @Date: 2022-04-25 23:46:20
 * @LastEditors: Please set LastEditors
 * @LastEditTime: 2022-04-25 23:53:21
 * @FilePath: \Rust_Tutorial\src\simple\ch_8_rc.rs
 * @Description:
 */
use std::rc::Rc;
#[test]
fn hello() {
    let word = "hello world".to_string();
    {
        println!("create a rc instance");
        let rc_a = Rc::new(word);
        println!("Reference counter of rc_a is {}", Rc::strong_count(&rc_a));
        {
            println!("create another rc instance");
            let rc_b = Rc::clone(&rc_a);
            println!("Reference counter of rc_a is {}", Rc::strong_count(&rc_a));
            println!("Reference counter of rc_b is {}", Rc::strong_count(&rc_b));
            assert_eq!(rc_a,rc_b);
            println!("Value of rc_b is {}", rc_b);
            println!("rc_b is dropped out of scope ");

        }
        println!("Now reference counter of rc_a is {}", Rc::strong_count(&rc_a));
        println!("rc_a is dropped out of scope ");
    }
    // assert_eq!("hello world", word);
}
