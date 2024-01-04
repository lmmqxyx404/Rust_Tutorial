/*
 * @Author: Lmmqxyx
 * @Date: 2022-04-07 18:13:26
 * @LastEditors: Please set LastEditors
 * @LastEditTime: 2022-07-06 06:50:41
 * @FilePath: \Rust_Tutorial\src\easy\ch_4_struct.rs
 * @Description:
 * struct initialize must bu named the key. also you can name a same name variable
 */
pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub struct line {
    pub start_point: Point,
    pub end_point: Point,
}

#[test]
fn valid() {
    let p = Point { x: 0.1, y: 0.5 };
    let mut ll = line {
        start_point: p,
        end_point: Point { x: 0.1, y: 0.5 },
    };
    ll.start_point.y = 2.2;
    assert_eq!(ll.start_point.x, 0.1);
}
