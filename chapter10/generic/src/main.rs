

#[derive(Debug)]
struct Point<T, U>
{
    x: T,
    y: U,
}

impl<T, U> Point<T, U>
{

    fn mixup<V, W>(self, point: Point<V, W>) -> Point<T, W>
    {
        return Point {
            x: self.x,
            y: point.y,
        }
    }
}


fn main() {

    let p1 = Point{x: 1, y: 5.0};
    let p2 = Point{x: "Hello", y: "good by"};

    let p3 = p1.mixup(p2);

    println!("P3 is {:#?}", p3);
}
