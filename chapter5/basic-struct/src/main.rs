


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

    println!("User info is {} with email {}, and it's {} and {}", u1.user, u1.email, u1.is_active, u1.count_of_sign);
}