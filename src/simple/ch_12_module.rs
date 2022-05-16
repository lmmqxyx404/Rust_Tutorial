/*
 * @Author: Lmmqxyx
 * @Date: 2022-05-16 17:40:40
 * @LastEditors: Please set LastEditors
 * @LastEditTime: 2022-05-16 17:46:44
 * @FilePath: \Rust_Tutorial\src\simple\ch_12_module.rs
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
}

fn function() {
    println!("called `function()`");
}
#[test]
fn hello() {
    function();
    assert_eq!("my_mod::function()", my_mod::function());
}
