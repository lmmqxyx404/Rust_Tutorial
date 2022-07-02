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

// Here is a designator expr ident
macro_rules! calculate {
    (eval $ee:expr) => {{
        {
            let val: usize = $ee;
            println!("{} = {}", $ee, val);
        }
    }};
}

macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() -> i32 {
            println!("This is a function named {:?}", stringify!($func_name));
            14
        }
    };
}

create_function!(my);
#[test]
fn middle_4_macro() {
    say_Hello!();
    calculate!(eval 1+2);
    assert_eq!(14, my());
}
