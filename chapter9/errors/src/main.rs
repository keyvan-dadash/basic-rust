

use std::io::ErrorKind;
use std::fs::File;

fn main() {
    
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(error) => panic!("Cannot create hello.txt with error {:?}", error)
            }
            other_error => {
                panic!("Cannot open hello.txt with error {:?}", other_error);
            }
        }
    };
}
