type FunctionType = for<'input> fn(input_string: &'input str) -> Result<(), &'input str>;

fn okay<'input>(_input_string: &'input str) -> Result<(), &'input str> {
    Ok(())
}

fn do_stuff_with_function(function: FunctionType) {
    let string = String::new();
    match function(string.as_str()) {
        Ok(_) => {},
        Err(_) => {},
    }

}

fn main() {
    do_stuff_with_function(okay);    
}