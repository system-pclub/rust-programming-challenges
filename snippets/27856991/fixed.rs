use std::ops::{Add};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

impl<'a> Add for &'a Point {
    type Output = Point;

    fn add(self, other: &'a Point) -> Point { //'
        Point {x: self.x + other.x, y: self.y + other.y}
    }
}

fn main() {
    let p: Point = Point {x: 1, y: 0};
    let pp = &p + &p;
    println!("{:?}", pp);
}