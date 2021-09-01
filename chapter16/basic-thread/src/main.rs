
use std::thread;
use std::time::Duration;

fn main() {
    

        

    let v1 = vec![1, 2, 3];

    let handler = std::thread::spawn(move || {
        println!("vector inside is {:#?}", v1);
    });

    handler.join().unwrap();
}
