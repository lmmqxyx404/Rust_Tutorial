macro_rules! m {
    ("$e") => {
        print!("e")
    };
    (1) => {
        print!("1")
    };
    ($tt:tt) => {
        print!("a")
    };
}

macro_rules! e {
    ($e:expr) => {
        m!($e)
    };
}

macro_rules! t {
    ($tt:tt) => {
        e!($tt);
        m!($tt);
    };
}

macro_rules! we {
    (2) => {
        print!("2")
    };
    ($tt:tt) => {
        e!($tt);
        m!($tt);
    };
}

#[test]
fn main() {
    t!{1};
    println!("");
    we!(2);
    println!("");
    we!(3);
    println!("");
    we!(2);
    println!("");
    we!(1);
}
