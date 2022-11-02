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
