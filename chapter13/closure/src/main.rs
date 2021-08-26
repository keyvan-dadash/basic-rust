

use std::thread;
use std::time::Duration;


struct Cacher<T>
where
    T: Fn(u64) -> u64
{
    calculation: T,
    value: Option<u64>,
}


impl<T> Cacher<T>
where
    T: Fn(u64) -> u64
{
    fn new(calculation: T) -> Cacher<T>
    {
        return Cacher {
            calculation: calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u64) -> u64 {
        match self.value {
            Some(v) => return v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                return v;
            }
        }
    }
}

fn example(number: u64) -> u64 {

    let mut closure = Cacher::new(|num| -> u64 {
        thread::sleep(Duration::from_secs(num));
        return num*2;
    });

    println!("going sleep for {}....", number);

    return closure.value(number);
}

fn main() {
    println!("Hello, world!");

    example(1);

    example(2);
}
