




fn main() {
    
    let v1 = vec![1, 2, 3];

    let v1_iter: Vec<_> = v1.iter().map(|x| x + 1).collect();

    let total: u32 = v1_iter.iter().sum();
    println!("sum of vector is {}", total);
}
