/*
 * @Author: Lmmqxyx
 * @Date: 2022-06-21 07:42:10
 * @LastEditors: Please set LastEditors
 * @LastEditTime: 2022-06-28 01:53:43
 * @FilePath: \Rust_Tutorial\src\middle\ch_1_deref.rs
 * @Description: 
 */
#[test]
fn middle_1_deref(){
    // pay aattention to the mutable reference
    let c="ccccc";
    let mut    calias=&c;
    calias=&"ddd";
    println!("This is middle {}",*calias);
} 