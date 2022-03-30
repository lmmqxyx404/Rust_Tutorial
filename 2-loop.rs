fn pass(mut arg:i64) {
    let mut counter:i64=0;
    loop {
        if arg%2==1 {
            arg =( arg * 3 + 1)/2;
        }else {
            arg=arg/2;
        }
        counter+=1;
        if arg == 1 {
            println!("{}",counter);
            return ;
        }
    };
}
fn main(){
    let args:Vec<String>=std::env::args().collect();
    // println!("{:?}",args);
    let arg:i64=args[1].parse::<i64>().unwrap();
    pass(arg);
    
    
}