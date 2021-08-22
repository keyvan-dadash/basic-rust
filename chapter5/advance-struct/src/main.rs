

struct User {
    user: String,
    email: String, 
    is_active: bool,
    count_of_sign: u64,
}

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

    println!("User info is {} with email {}, and it's {}", u2.user, u2.email, u2.is_active)
}