#[derive(Debug)]
struct Point(f64, f64);

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Point(x, y)
    }
    fn origin() -> Self {
        Point(0., 0.)
    }
    fn distance_to(&self, other: &Point) -> f64 {
        ((other.0 - self.0).powi(2) + (other.1 - self.1).powi(2)).sqrt()
    }
}

fn task(x: f64, y: f64) -> f64 {
    let a = Point::new(x, y);
    let b = a;
    let d = a.distance_to(&b);
    d
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        assert!((task(2.5, 1.0)).abs() < 1e-5);
    }
}
