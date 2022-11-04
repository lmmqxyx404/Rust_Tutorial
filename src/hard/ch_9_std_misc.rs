use std::fs::File;
use std::path::Path;
use std::thread;
use std::time::Duration;

#[test]
fn hello() {
    let mut children_thread = vec![];
    for i in 0..10 {
        children_thread.push(thread::spawn(move || {
            println!("thread {}", i);
        }));
        thread::sleep(Duration::from_millis(1000));
    }

    for child in children_thread {
        let _ = child.join();
    }
}

#[test]
fn path() {
    let path = Path::new("hello.txt");
    let display = path.display();
    let mut file = match File::open(path) {
        Err(why) => panic!("could not open the file."),
        Ok(file) => file,
    };
}
