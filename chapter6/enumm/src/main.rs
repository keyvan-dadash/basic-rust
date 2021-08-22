

enum IP {
    Ipv4(u8, u8, u8, u8),
    Ipv6(String),
}



fn main() {
    

    let v4 = IP::Ipv4;
    let v6 = IP::Ipv6;
}
