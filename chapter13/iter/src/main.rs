




fn main() {
    
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    let value = v1_iter.next();
    println!("value is {:#?}", value);

    let value = v1_iter.next();
    println!("value is {:#?}", value);

    let value = v1_iter.next();
    println!("value is {:#?}", value);
}
