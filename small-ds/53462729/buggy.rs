struct MyStruct<'a, 'b> {
    pub source: &'a str,
    pub state: &'b mut Vec<&'b str>,
}

fn func(arg: MyStruct) {
    println!("{}", arg.source);
    println!("{}", arg.state[0]);
}

fn step<'a>(source: &'a str,
            state: &mut Vec<&'a str>) {
    state.push(&source[4..10]);
    let s = MyStruct{source: source, state: state};
    func(s);
    state.push(&source[4..10]);
}

fn main() {
    let source = "abcdefghijklmnopqrstuvwxyz";
    {
        let mut state = Vec::<&str>::new();
        step(source, &mut state);
        step(source, &mut state);
        step(source, &mut state);
    }
}