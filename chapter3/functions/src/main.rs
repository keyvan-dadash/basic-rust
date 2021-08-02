fn main() {
    println!("Hello, world!");

    let y = {
        let mut x = 4;
        x *= 4;
        x + 1
    };

    println!("Another function. {}", y);

    another_function(3);
}

fn another_function(x: i32) {
    println!("Another function. {}", x);
}
