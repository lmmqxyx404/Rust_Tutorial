/*
 * @Author: Lmmqxyx
 * @Date: 2022-06-21 07:42:10
 * @LastEditors: Please set LastEditors
 * @LastEditTime: 2022-06-28 00:00:49
 * @FilePath: \Rust_Tutorial\src\middle\ch_0_hello.rs
 * @Description:
 */

macro_rules! say_Hello {
    () => {
        println!("hello rust macro");
    };
}

macro_rules! calculate {
    (eval $ee:expr) => {{
        {
            let val: usize = $ee;
            println!("{} = {}", $ee, val);
        }
    }};
}
#[test]
fn middle_4_macro() {
    say_Hello!();
    calculate!(eval 1+2);
}
