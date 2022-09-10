use std::thread;
use std::time::Duration;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello() {
        let handler = thread::spawn(|| {
            for i in 1..10 {
                println!("{} thread", i);
                thread::sleep(Duration::from_millis(50));
            }
        });
        for i in 100..105 {
            println!("{} thread", i);
            thread::sleep(Duration::from_millis(100));
        }
        handler.join().unwrap();
    }

    #[test]
    fn hello_2() {
        let handler = thread::spawn(|| {
            for i in 1..10 {
                println!("{} thread", i);
                thread::sleep(Duration::from_millis(50));
            }
        });
        handler.join().unwrap();
        for i in 100..105 {
            println!("{} thread", i);
            thread::sleep(Duration::from_millis(100));
        }
    }
}
