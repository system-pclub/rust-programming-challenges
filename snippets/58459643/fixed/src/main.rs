use crossbeam; // 0.7.3

struct Settings {
    // ... many fields
}

const MAX_FEASIBLE_SCORE: u8 = 10;

fn example(settings: Settings) {
    let settings = &settings;
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

fn do_cool_computation(_: &Settings, _: u8) {}

fn main() {}