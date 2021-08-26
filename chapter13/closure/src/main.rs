

use std::thread;
use std::time::Duration;

fn example(number: u64) -> u64 {

    let closure = |num| -> u64 {
        thread::sleep(Duration::from_secs(num));
        return num*2;
    };

    println!("going sleep for {}....", number);

    return closure(number);
}

fn main() {
    println!("Hello, world!");

    example(1);

    example(2);
}
