
#[derive(Debug)]
struct Rectangle {
    width: i32,
    heigth: i32,
}

impl Rectangle {

    fn area(&self) -> i32 {
        self.width * self.heigth
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return (self.width > other.width) && (self.heigth > other.heigth)
    }
}

impl Rectangle {

    fn square(size: i32) -> Rectangle {
        Rectangle {
            heigth: size,
            width: size,
        }
    }
}

fn main() {

    let r = Rectangle {
        width: 5,
        heigth: 10,
    };

    println!("rect area is {}", r.area());

    let s = Rectangle::square(5);

    println!("square area is {}", s.area());

    println!("rect and square fit? {}", r.can_hold(&s));
}
