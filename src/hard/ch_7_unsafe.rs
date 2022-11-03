static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
#[test]
fn hello() {
    add_to_count(12);
    println!("Hard trip starts.")
}

#[test]
fn raw_pointer() {
    let mut num = 5;
    let raw_pointer_1 = &num as *const i32;
    let raw_pointer_2 = &mut num as *mut i32;
    unsafe {
        assert_eq!(*raw_pointer_1, 5);
        *raw_pointer_2 = *raw_pointer_2 + 10;
        assert_eq!(*raw_pointer_2, 15);
    }
    // println!("{}", raw_pointer_1);
}
