




fn main() {
    
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: u32 = v1_iter.sum();
    println!("sum of vector is {}", total);
}
