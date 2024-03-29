use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

mod tests {
    use super::*;

    #[test]
    fn hello() {
        let handler = thread::spawn(|| {
            for i in 1..10 {
                println!("{} spawned thread", i);
                thread::sleep(Duration::from_millis(500));
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
                println!("{} spawned thread", i);
                thread::sleep(Duration::from_millis(500));
            }
        });
        // the following code would be blocked the main thread
        handler.join().unwrap();
        for i in 100..105 {
            println!("{} thread", i);
            thread::sleep(Duration::from_millis(100));
        }
    }
    #[test]
    fn move_3() {
        let vv = vec![1, 2, 3];
        // pay attention to the move keyword
        let handler = thread::spawn(move || {
            for i in 0..vv.len() {
                println!("{} spawned thread ", i);
                thread::sleep(Duration::from_millis(50));
            }
        });
        handler.join().unwrap();
        for i in 100..105 {
            println!("{} thread", i);
            thread::sleep(Duration::from_millis(100));
        }
    }

    #[test]
    fn mpsc_4() {
        // pay attention to the mpsc that send() returns Ok() not means that the receiver can receive the relative information.
        let (tx, rx) = mpsc::channel();
        let handler = thread::spawn(move || {
            thread::sleep(Duration::from_millis(10000));
            for i in 1..10 {
                println!("sended: {}", i);
                match tx.send(i * i) {
                    Ok(_) => {
                        println!("Yes the data has been sent successfully");
                    }
                    Err(e) => {
                        println!("failed to send the data");
                    }
                };
                thread::sleep(Duration::from_millis(1000));
                // tx.send(i * i).unwrap();
                // thread::sleep(Duration::from_millis(2000));
            }
        });
        // understand the err branch reason.
        loop {
            match rx.recv() {
                Ok(val) => {
                    println!("rec:{}", val);
                }
                Err(_) => {
                    panic!("can not accept val");
                }
            }
        }
    }

    #[test]
    fn concunrency_5() {
        let m = Mutex::new(5);
        {
            let mut num = m.lock().unwrap();
            *num = 100;
        }
        println!("mu ={:?}", m);
    }

    #[test]
    fn concunrency_6_mutex_arc() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }
        // can read the intial value,but can not guarantee the 
        thread::sleep(Duration::from_secs(2));
        println!("Result is: {}", *counter.lock().unwrap());

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result is: {}", *counter.lock().unwrap());
    }
}
