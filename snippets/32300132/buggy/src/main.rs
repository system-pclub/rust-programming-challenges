struct Thing {
    count: u32,
}

struct Combined<'a>(Thing, &'a Thing);

fn make_combined<'a>() -> Combined<'a> {
    let thing = Thing {count: 0};

    Combined(thing, &thing)
}

fn main() {}
