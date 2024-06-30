use crossbeam; // 0.7.3

struct Settings {
    _a: u8,
    _b: u8,
    _c: u8,
}

const MAX_FEASIBLE_SCORE: u8 = 10;

fn example(settings: Settings) {
    crossbeam::scope(|scope| {
        for score in 0..MAX_FEASIBLE_SCORE {
            scope.spawn(move |_| {
                let work_result = do_cool_computation(&settings, score);
                println!("{:?}", work_result);
            });
        }
    })
    .unwrap();
}

fn do_cool_computation(settings: &Settings, score: u8) -> u8 {
    settings._a + settings._b + settings._c + score
}

fn main() {}