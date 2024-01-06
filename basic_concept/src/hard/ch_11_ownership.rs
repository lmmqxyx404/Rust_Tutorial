struct Book {
    pub name: String,
    pub index: i32,
}

fn print_book_name(b: &mut Book) {
    // can not compile.
    // let name = b.name;
    // println!("the book name is {}", name);
    let num = b.index;
    assert_eq!(num, 45);
}

#[test]
pub fn ch_11_ownership() {
    let mut b = Book {
        name: "Great".to_owned(),
        index: 45,
    };
    print_book_name(&mut b);
    let c = b.name;
    assert_eq!("Great", c);
    // assert_eq!("Great", b.name);
    assert_eq!(45, b.index);
}
