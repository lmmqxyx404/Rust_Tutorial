/*
 * @Author: Lmmqxyx
 * @Date: 2022-05-16 17:40:40
 * @LastEditors: Please set LastEditors
 * @LastEditTime: 2022-07-06 06:51:24
 * @FilePath: \Rust_Tutorial\src\easy\ch_12_module.rs
 * @Description:
 */
mod my_mod {
    fn private_func() {
        println!("This is a private function");
    }

    pub fn function() -> String {
        String::from("my_mod::function()")
    }

    pub fn indirect_access() {
        print!("called `hello` ");
        private_func();
    }
    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }

        pub(in crate::easy::ch_12_module::my_mod) fn public_function_in_my_mod() {
            println!("hello");
            public_function_in_nested();
        }

        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested()`");
        }

        pub(super) fn public_function_in_super_mod() {
            println!("called `my_mod::nested::public_function_in_super_mod()`");
        }
    }
    pub fn call_public_function_in_my_mod() {
        print!("called `my_mod::call_public_function_in_my_mod()`,that\n>");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()`");
    }
}

fn function() {
    println!("called root `function()`");
}
#[test]
fn hello() {
    function();
    assert_eq!("my_mod::function()", my_mod::function());
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();
    my_mod::public_function_in_crate();

}
