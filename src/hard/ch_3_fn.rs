/*
 * @Author: Lmmqxyx
 * @Date: 2022-04-07 18:07:48
 * @LastEditors: Please set LastEditors
 * @LastEditTime: 2022-07-07 22:48:27
 * @FilePath: \Rust_Tutorial\src\hard\ch_0_hello.rs
 * @Description:
 */
#[allow(unused)]
fn do_twice<F>(mut fnc: F)
where
    F: FnMut(),
{
    fnc();
    fnc();
}

#[test]
fn ch_3_fn() {
    let mut x = 4;
    let add = || x += 2;
    do_twice(add);
    assert_eq!(x, 8)
}
