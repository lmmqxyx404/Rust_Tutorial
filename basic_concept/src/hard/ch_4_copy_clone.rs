#[derive(Debug)]
pub struct Node {
    pub val: i32,
}

impl Node {
    pub fn new(val: i32) -> Self {
        Node { val }
    }
}
#[test]
fn main() {
    // println!("{:}", Node::new(5)); must add Debug trait
    println!("{:?}", Node::new(5));

    let a = Node::new(5);
    let b = a;
    // a has been moved to b.
    // assert_eq!(b.val,a.val);
    let dd = &b;
    // automatic dereference
    assert_eq!(b.val, (*dd).val);
    assert_eq!(b.val, dd.val);

    let aa = 5;
    let ss = aa;
    // because i32 has Copy Trait
    assert_eq!(aa, ss);
    let ss = &aa;
    // assert_eq!(aa, ss);
}
