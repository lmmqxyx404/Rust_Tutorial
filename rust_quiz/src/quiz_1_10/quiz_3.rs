struct S {
    x: i32,
}

const S: S = S { x: 2 };

/// The semantics of const is that any mention of the const by name in expression position is substituted with the value of the const initializer.
#[test]
fn main() {
    let v = &mut S;
    assert_eq!(v.x, S.x);
    v.x += 1;
    assert_eq!(v.x, 3);
    S.x += 1;
    assert_eq!(S.x, 2);
    print!("{}{}", v.x, S.x);
}
