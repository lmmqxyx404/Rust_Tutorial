struct ListNode<T> {
    val: T,
    next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
    fn new(val: T) -> Self {
        ListNode { val, next: None }
    }
}

#[test]
fn hello() {
    println!("Hard trip starts.");
    let mut head = Some(Box::new(ListNode::new(5)));
    let head2 = ListNode::new(45);
    // get mutable ref
    head.as_mut().unwrap().next = Some(Box::new(head2));
    assert_eq!(head.unwrap().next.unwrap().val, 45);
}
