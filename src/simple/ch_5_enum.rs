/*
 * @Author: Lmmqxyx
 * @Date: 2022-04-11 13:44:03
 * @LastEditors: Please set LastEditors
 * @LastEditTime: 2022-04-11 15:16:46
 * @FilePath: \Rust_Tutorial\src\simple\ch_5_enum.rs
 * @Description:
 */
enum WebEvent {
    PageLoad,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}
enum Status {
    Poor,
    Rich
}

enum Work {
    Soldier,
    Engineer
}
struct Person {
    status:Status,
    work:Work,
}
fn inspect(event: WebEvent) -> String {
    match event {
        WebEvent::Paste(s) => s,
        _ => String::from("Not Paste"),
    }
}
#[test]
fn valid() {
    type WE = WebEvent;
    let p = WE::Paste(String::from("hello"));
    use crate::simple::ch_5_enum::Status::{Poor,Rich};
    use crate::simple::ch_5_enum::Work::*;
    let me=Person {status:Poor,work:Engineer};
    match me.status {
        Poor =>{
            println!("low b status:{}, work:{}",me.status as u8,me.work as u8);
        },
        _=>{
            println!("boos");
        }
    }
    assert_eq!(inspect(p), String::from("hello"));
}
