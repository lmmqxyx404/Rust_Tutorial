#[test]
fn hello() {
    let word = "hello world".to_string();
    let mut num = (5..10).collect::<Vec<i32>>();
    println!("Collected numbers are {:?}", num);
    for (x, index) in num.iter().enumerate() {
        println!("Collected numbers are {:?} {:?}", x, index);
    }
    for x in num.iter_mut() {
        *x = *x * 3;
        println!("Collected numbers are {:?}", x);
    }
}
