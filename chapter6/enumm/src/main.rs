


#[derive(Debug)]
enum IP {
    Ipv4(u8, u8, u8, u8),
    Ipv6(String),
}



fn main() {
    

    let v4 = IP::Ipv4(127, 0, 0, 1);
    let v6 = IP::Ipv6(String::from("::1"));

    println!("ipv4 is {:#?} and ipv6 is {:#?}", v4, v6);
}
