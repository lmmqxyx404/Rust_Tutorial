/*
 * @Author: Lmmqxyx
 * @Date: 2022-04-07 18:07:48
 * @LastEditors: Please set LastEditors
 * @LastEditTime: 2022-07-07 22:57:45
 * @FilePath: \Rust_Tutorial\src\hard\ch_1_chain.rs
 * @Description:
 */
// normal chain call
#[derive(Debug)]
struct Student {
    name: String,
    age: u8,
}

impl Student {
    fn new() -> Student {
        Student {
            name: "xuji".to_string(),
            age: 11,
        }
    }
    fn introduce_myself(&self) {
        println!("{:?}", self);
    }
    fn say_name(&self) -> &Self {
        println!("My name is {}", self.name);
        self
    }
    fn say_age(&self) -> &Self {
        println!("My age is {}", self.age);
        self
    }
}
#[test]
fn ch_1_chain() {
    let xuji = Student::new();
    xuji.say_name().say_age().introduce_myself();
}
