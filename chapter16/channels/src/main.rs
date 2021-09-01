
use std::thread;
use std::sync::mpsc;


fn main() {
    
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("from inside channel");
        tx.send(val).unwrap();
    });

    let result = rx.recv();

    match result {
        Ok(value) => {
            println!("good it's {}", value);
        },
        Err(err) => {
            println!("error is {}", err);
        }
    }


}
