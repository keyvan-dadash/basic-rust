



enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn valu_in_cent(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {

    let s = Coin::Penny;

    println!("valu of penny is {}", valu_in_cent(s));
}
