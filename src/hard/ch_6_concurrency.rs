use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[cfg(test)]
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
        let (tx, rx) = mpsc::channel();
        let handler = thread::spawn(move || {
            for i in 1..10 {
                println!("sended: {}", i);
                tx.send(i * i).unwrap();
                // thread::sleep(Duration::from_millis(2000));
            }
        });

        for i in 1..10 {
            thread::sleep(Duration::from_millis(1000));
            println!("rec:{}", rx.recv().unwrap());
        }
    }

    #[test]
    fn concunrency_5() {
        use std::sync::Mutex;
        let m = Mutex::new(5);
        {
            let mut num = m.lock().unwrap();
            *num = 100;
        }
        println!("mu ={:?}", m);
    }
}
