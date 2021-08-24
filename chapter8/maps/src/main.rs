

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

    for (keyy, valuee) in &map {
        println!("value {} is for key {}", valuee, keyy);
    }

    //overwrite value
    map.insert(String::from("Key"), 10);

    map.entry("Key2".to_string()).or_insert(20);
    map.entry("Key".to_string()).or_insert(20);

    println!("map is {:#?}", map);

    let number = map.entry("Key".to_string()).or_insert(20);
    *number *= 5;

    println!("map is {:#?}", map);
}
