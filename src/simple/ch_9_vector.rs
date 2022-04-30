#[test]
fn hello() {
    let word = "hello world".to_string();
    let num = (0..10).collect::<Vec<i32>>();
    println!("Collected numbers are {:?}", num);
    for x in num.iter() {
        println!("Collected numbers are {:?}", x);
    }
}
