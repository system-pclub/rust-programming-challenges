type Real = f64;

pub fn outer<'a, F>(f1: &'a F) -> Box<dyn Fn(&Real) -> Real + 'a>
    where F: Fn(&Real) -> Real
{
    Box::new(move |x| f1(x) + f1(x))
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_outer() {
        let f = |x: &Real| x * 2.0;
        let g = outer(&f);
        assert!((g(&3.0) - 12.0).abs() < 1e-5);
    }
}