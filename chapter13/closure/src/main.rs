

use std::thread;
use std::time::Duration;
use std::collections::HashMap;

struct Cacher<T>
where
    T: Fn(u64) -> u64
{
    calculation: T,
    map: HashMap<u64, u64>,
}


impl<T> Cacher<T>
where
    T: Fn(u64) -> u64
{
    fn new(calculation: T) -> Cacher<T>
    {
        return Cacher {
            calculation: calculation,
            map: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u64) -> u64 {
        match self.map.get(&arg) {
            Some(v) => return *v,
            None => {
                let v = (self.calculation)(arg);
                self.map.insert(arg, v);
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
