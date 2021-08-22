
#[derive(Debug)]
struct Rectangle {
    width: i32,
    heigth: i32,
}

fn main() {

    let r = Rectangle {
        width: 5,
        heigth: 10,
    };

    println!("Rest is {:#?}", r);
}
