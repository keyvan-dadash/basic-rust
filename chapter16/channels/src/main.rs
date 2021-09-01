
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        thread::sleep(std::time::Duration::from_millis(3));
        for val in 1..10 {
            let val : u32 = val;
            tx1.send(val.to_string()).unwrap();
        }
    });

    thread::spawn(move || {
        for val in 11..21 {
            let val : u32 = val;
            tx.send(val.to_string()).unwrap();
        }
    });

    for result in rx {
        println!("good it's {}", result);
    }
}
