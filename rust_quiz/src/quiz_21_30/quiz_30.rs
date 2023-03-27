


use std::rc::Rc;

struct A;

fn p<X>(x: X) {
    println!("{}", std::mem::size_of::<X>());
    /* match std::mem::size_of::<X>() {
        0 => println!("0"),
        _ => println!("1"),
    } */
}

/// About clone and basic size of struct and ().
#[test]
fn main() {
    let a = &A;
    p(a);
    p(a.clone());

    let b = &();
    p(b);
    p(b.clone());

    let c = Rc::new(());

    p(Rc::clone(&c));
    p(c.clone());
    p(c);

    let aa = A;
    assert_eq!(std::mem::size_of::<A>(), 0);
}
