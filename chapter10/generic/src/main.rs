


fn simple<T>(list: &[T]) -> T
{
    let largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}


fn main() {
    println!("Hello, world!");
}
