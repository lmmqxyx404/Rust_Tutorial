/*
 * @Author: Lmmqxyx
 * @Date: 2022-06-21 07:42:10
 * @LastEditors: Please set LastEditors
 * @LastEditTime: 2022-06-28 00:00:49
 * @FilePath: \Rust_Tutorial\src\middle\ch_0_hello.rs
 * @Description:
 */
use std::{thread, time::Duration};
#[test]
fn middle_thread() {
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("{} from the spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("main thread {}", i);
        thread::sleep(Duration::from_millis(100));
    }
    println!("The module is about middle knowledge");
    handle.join().unwrap();
}
