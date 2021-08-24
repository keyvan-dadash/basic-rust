



fn main() {
    

    let data = "New String";

    let new_str = data.to_string();

    println!("string from literal {}", new_str);

    let mut new_1str = String::from("This is ");

    new_1str.push_str(&new_str);

    println!("New string is {}", new_1str);

    let s1 = String::from("Hello, ");
    let s2 = String::from("World");

    let s3 = s1 + &s2;

    println!("New string after operator + is {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    println!("String after format macro is {}", s);

    for c in s.chars() {
        println!("char is {}", c);
    }


}
