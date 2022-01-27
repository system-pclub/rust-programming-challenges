#[derive(Debug)]
struct Point(f64, f64);

impl Point {
    fn new(x: f64, y: f64) -> Self { Point(x, y) }
    fn origin() -> Self { Point(0.,0.) }
    fn distance_to(&self, other: &Point) -> f64 {
        ((other.0 - self.0).powi(2) + (other.1 - self.1).powi(2)).sqrt()
    }
}

fn main() {
    let grid_size = 5;

    let points_in_grid = (0..grid_size).flat_map(|x| {
        (0..grid_size)
            .map(|y| Point::new(f64::from(x), f64::from(y)))
            .collect::<Vec<Point>>()
    });

    let origin = Point::origin();

    let points_and_distances = points_in_grid
        .map(|point| {
            let d = point.distance_to(&origin);
            (point, d)
        })
        .collect::<Vec<(Point, f64)>>();
}