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

use crate::easy::ch_4_struct::*;

#[test]
fn ch_8_refcell_again() {
    let p = Point { x: 0.1, y: 0.5 };
    assert_eq!(p.x, 0.1);
    /* let mut ll = line {
        start_point: p,
        end_point: Point { x: 0.1, y: 0.5 },
    };
     ll.start_point.y = 2.2; */
}
