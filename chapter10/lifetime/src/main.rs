



fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    
    let string1 = String::from("Hellooooooo");
    let string2 = "sdlkfj";

    let resutl = longest(&string1, string2);
    println!("Longest string is {}", resutl);
}
