

use std::collections::HashMap;

fn main() {
    
    let mut map = HashMap::new();

    map.insert(String::from("Key"), 5);
    map.insert(String::from("Key1"), 10);

    let key = String::from("Key");

    match map.get(&key) {
        Some(value) => {
            println!("value for key is {}", value);
        },
        None => {
            println!("value does not exists");
        }
    }

    let invalid_key = String::from("sdlkjf");
    match map.get(&invalid_key) {
        Some(value) => {
            println!("value for key is {}", value);
        },
        None => {
            println!("value does not exists");
        }
    }
}