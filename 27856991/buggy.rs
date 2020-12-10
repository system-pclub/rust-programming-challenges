use std::ops::{Add};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {x: self.x + other.x, y: self.y + other.y}
    }
}

fn main() {
    let p: Point = Point {x: 1, y: 0};
    let pp = p + p;
}
