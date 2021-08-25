



fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


#[derive(Debug)]
struct ImportantExcerpt<'a>
{
    x: &'a str
}

fn main() {
    
    let string1 = String::from("long string is long");

    let is_valid_struct;
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);

        is_valid_struct = ImportantExcerpt {
            x: string1.as_str(),
        }
    }

    println!("content of struct is {:#?}", is_valid_struct);


}
