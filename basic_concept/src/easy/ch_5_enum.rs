/*
 * @Author: Lmmqxyx
 * @Date: 2022-04-11 13:44:03
 * @LastEditors: Please set LastEditors
 * @LastEditTime: 2022-04-13 17:47:40
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
    Rich,
}

enum Work {
    Soldier,
    Engineer,
}
struct Person {
    status: Status,
    work: Work,
}
fn inspect(event: WebEvent) -> String {
    match event {
        WebEvent::Paste(s) => s,
        _ => String::from("Not Paste"),
    }
}
#[test]
fn valid() {
    // you can rename former custom type.
    type WE = WebEvent;
    let p = WE::Paste(String::from("hello"));
    // path
    use crate::easy::ch_5_enum::Status::{Poor, Rich};
    use crate::easy::ch_5_enum::Work::*;
    let me = Person {
        status: Poor,
        work: Engineer,
    };
    match me.status {
        Poor => {
            println!("low b status:{}", me.status as u8);
            // also enum started with 0
            assert_eq!(1, me.work as u8);
        }
        _ => {
            println!("boos");
        }
    }
    assert_eq!(inspect(p), String::from("hello"));
}
