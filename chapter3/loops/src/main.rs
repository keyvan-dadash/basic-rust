fn main() {
    
    let mut g = 0;

    let resutl = loop {
        g += 1;

        if g >= 10 {
            break g * 4;
        }

        println!("fuck youuuuuuuuuuuuu");
    };

    println!("after loop got {}", resutl);
}
