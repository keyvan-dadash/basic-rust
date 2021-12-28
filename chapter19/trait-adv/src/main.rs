use std::ops::Add;


#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

struct Point2 {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        return Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
} 

impl Add<Point2> for Point {
    type Output = Point;

    fn add(self, other: Point2) -> Point {
        return Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
} 



fn main() {

    assert_eq!(
        Point{ x: 1, y: 2 } + Point{ x: 2, y: 4 },
        Point{ x: 3, y: 6 }
    );

    assert_eq!(
        Point{ x: 1, y: 2 } + Point2{ x: 2, y: 4 },
        Point{ x: 3, y: 6 }
    );
}
