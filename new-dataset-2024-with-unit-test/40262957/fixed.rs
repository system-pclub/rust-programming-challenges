struct BigData {
    data: i32,
}

struct Data {
    x: Box<BigData>,
}

fn calc_data(x: &Box<BigData>) -> i32 {
    x.data + 42
}

fn task(input: i32) -> String {
    let b = BigData { data: input };
    let d = Data { x: Box::new(b) };
    let borrowed_d = &d;
    let ret = calc_data(&borrowed_d.x);
    format!("{}", ret)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        assert_eq!(task(42), "84");
    }
}
