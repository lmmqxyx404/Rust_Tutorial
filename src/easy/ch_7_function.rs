/*
 * @Author: Lmmqxyx
 * @Date: 2022-04-23 23:32:36
 * @LastEditors: Please set LastEditors
 * @LastEditTime: 2022-04-23 23:46:41
 * @FilePath: \Rust_Tutorial\src\simple\ch_7_function.rs
 * @Description:
 */
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }
    // you can omit the specific variable
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

struct Rectangle {
    leftTop:Point,
    rightBottom:Point,
}

impl Rectangle {
    // calculate area
    fn area(&self) -> f64 {
        let Point{x,y}=self.leftTop;
        let Point{x:x2,y:y2}=self.rightBottom;
        ((x-x2)*(y-y2)).abs()
    }
    // calculate perimeter
    fn perimeter(&self) -> f64 {
        let Point{x,y}=self.leftTop;
        let Point{x:x2,y:y2}=self.rightBottom;
        ((x-x2).abs()+(y-y2).abs())*2.0
    }
}
#[test]
fn main() {
    // let word = "hello world";
    let rect = Rectangle{
        leftTop: Point::origin(),
        rightBottom: Point::new(3.0,5.0)
    };
    assert_eq!(rect.area(), 15.0);
    assert_eq!(rect.perimeter(), 16.0);
}
