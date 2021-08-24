


fn main() {


    let mut vec1 = vec![1, 2, 3];
    
    //push back
    vec1.push(4);
    vec1.push(5);
    vec1.push(6);

    //get by index

    let third : &i32 = &vec1[2];

    println!("We have third index(by []) which is {}", third);


    match vec1.get(2) {
        Some(third) => {
            println!("We have third index which is {}", third);
        },
        None => {
            println!("We dont have third index");
        }
    }

    match vec1.get(7) {
        Some(seven) => {
            println!("We have seven index which is {}", seven);
        },
        None => {
            println!("We dont have seven index");
        }
    }

    for i in &vec1 {
        println!("elemenet {}", i);
    }
}
