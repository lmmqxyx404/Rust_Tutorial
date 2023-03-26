/*
 * @Author: Lmmqxyx
 * @Date: 2022-04-07 18:07:48
 * @LastEditors: Please set LastEditors
 * @LastEditTime: 2022-07-07 22:48:27
 * @FilePath: \Rust_Tutorial\src\hard\ch_0_hello.rs
 * @Description:
 */

struct about_pointer {
    aligned: u8,
    unaligned: u32,
}
#[test]
fn pointer_1() {
    let mut num = 1;
    let b = &num as *const i32;
    let c = &mut num as *mut i32;
    // can not deref directly.
    // println!("{}", *b);
    let cc = Box::new(45);
    let dd = Box::into_raw(cc);
    unsafe {
        println!("{}", *dd);
    }
}

#[test]
fn pointer_2() {
    let mut pp = about_pointer {
        aligned: 12,
        unaligned: 122,
    };
    let mut apointer = &pp.unaligned;
    assert_eq!(*apointer, 122);
    pp.unaligned = 321456;
    // assignment to borrowed `pp.unaligned`.
    // apointer = &(*apointer + 124);
    pp.unaligned = 4569;
    // assert_eq!(*apointer, 321456);
    // can generate a mutable raw pointer.
    let p = std::ptr::addr_of_mut!(pp.unaligned);
    unsafe {
        *p += 1;
        assert_eq!(*p, 4570);
    }
    pp.unaligned = 123;
    unsafe {
        assert_eq!(*p, 123);
    }
}
