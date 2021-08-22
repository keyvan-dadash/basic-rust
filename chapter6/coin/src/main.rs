


#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn valu_in_cent(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Penny!!");
            1
        },
        Coin::Quarter(state) => {
            println!("this coing belong to {:#?}", state);
            25
        },
        _ => 11,
    }
}

fn main() {

    let s = Coin::Quarter(UsState::Alabama);

    println!("value of quarter is {}", valu_in_cent(s));


    let y = Coin::Quarter(UsState::Alaska);

    if let Coin::Quarter(state) = y {
        println!("hello {:#?}", state);
    } else {
        println!("okkkkk");
    }


    let l = Coin::Dime;

    println!("value of unkown is {}", valu_in_cent(l));
}
