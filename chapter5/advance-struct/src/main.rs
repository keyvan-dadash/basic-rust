

struct User {
    user: String,
    email: String, 
    is_active: bool,
    count_of_sign: u64,
}

struct Color(i32, i32, i32);

fn main() {


    let u1 = User {
        user: String::from("Hoho"),
        email: String::from("Hoho@hoho.com"),
        is_active: true,
        count_of_sign: 1,
    };


    let u2 = User {
        user: String::from("Hoh1o"),
        email: String::from("Hoho@hoh1o.com"),
        ..u1
    };

    let black = Color(0, 0, 0);

    println!("Black is {}, {}, {}", black.0, black.1, black.2);

    println!("User info is {} with email {}, and it's {}, and coutn sign {}", u2.user, u2.email, u2.is_active, u2.count_of_sign);
}