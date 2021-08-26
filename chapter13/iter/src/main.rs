


struct Counter {
    count: u32,
}

impl Counter {

    fn new() -> Counter {
        return Counter {
            count: 0,
        }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> 
    {
        if self.count < 5 {
            self.count += 1;
            return Some(self.count);
        } else {
            return None;
        }
    }
}


fn main() {
    
    // let v1 = vec![1, 2, 3];

    // let v1_iter: Vec<_> = v1.iter().map(|x| x + 1).collect();

    // let total: u32 = v1_iter.iter().sum();
    // println!("sum of vector is {}", total);

    let mut counter = Counter::new();

    println!("vlaue is {:#?}", counter.next());
    println!("vlaue is {:#?}", counter.next());
    println!("vlaue is {:#?}", counter.next());
    println!("vlaue is {:#?}", counter.next());
    println!("vlaue is {:#?}", counter.next());
}
