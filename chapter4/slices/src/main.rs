





fn main() {
    
    let t = String::from("Hello, there are u okey");

    let tt = first_word(&t);

    println!("word is {}", tt)
}


fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if b' ' == item {
            return &s[..i]
        }
    }

    &s
}
