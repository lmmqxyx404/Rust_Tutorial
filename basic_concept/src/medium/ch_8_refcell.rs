use std::{cell::RefCell, rc::Rc};

#[allow(unused)]
struct TreeNode {
    val: i32,
    /* left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>, */
}
#[allow(unused)]
impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            /* left: None,
            right: None, */
        }
    }
}
#[test]
fn ch_8_refcell() {
    let a = Rc::new(5);
    let b = Rc::new(RefCell::new(6));
    let c = *b.borrow();
    let d = *b.borrow();
    assert_eq!(5, *a);
    assert_eq!(c, d);

    let aa = Rc::new(RefCell::new(TreeNode::new(15)));
    let ss = &*aa.borrow();
    assert_eq!(15, (*ss).val);

    let aa = &Rc::new(RefCell::new(TreeNode::new(15)));
    let aval = aa.borrow().val;
    assert_eq!(15, aval);

    let root = TreeNode::new(6);
    let pnode = Some(root);
    // assert_eq!(6, pnode.clone().unwrap().val);
    assert_eq!(6, pnode.unwrap().val);
    // assert_eq!(6, root.val);

    // get the reference of a tree root
    let node = TreeNode::new(12);
    let aa = Some(Rc::new(RefCell::new(node)));
    // can not get the node 42 ownership
    // assert_eq!(12,node.val);
    let aval = aa.as_ref().unwrap().borrow().val;
    assert_eq!(12, aval);
    let bb = aa.unwrap();
    let dd = &*bb.borrow();

    // let aval=aa.as_ref().b;
    // let dd = aa;
    // assert_eq!(dd.take(), 0)
    // assert_eq!(6, );
}

#[derive(Clone, Copy)]
struct Point {
    pub x: f32,
    pub y: f32,
}

struct line {
    pub start_point: Rc<RefCell<Point>>,
    pub end_point: Rc<RefCell<Point>>,
}
#[test]
fn ch_8_refcell_again() {
    let p = Rc::new(RefCell::new(Point { x: 0.1, y: 0.5 }));

    let mut ll = line {
        start_point: p.clone(),
        end_point: p.clone(),
    };
    assert_eq!(p.borrow().x, 0.1);
    let mut q = (*p.borrow()).clone();
    q.x = 0.2;
    assert_eq!(p.borrow().x, 0.1);
    ll.end_point.borrow_mut().y = 2.2;
    assert_eq!(ll.end_point.borrow().y, 2.2);
    p.borrow_mut().y = 2.3;
    assert_eq!(ll.end_point.borrow().y, 2.3);
}

#[test]
fn ch_8_rc_again() {
    let mut p = Rc::new(Point { x: 0.1, y: 0.5 });
    assert_eq!(p.x, 0.1);
    let mut q = p.clone();
    assert_eq!(p.x, q.x);
    
    p = Rc::new(*q);
    // assert_eq!(q.x, 2.2);
}
