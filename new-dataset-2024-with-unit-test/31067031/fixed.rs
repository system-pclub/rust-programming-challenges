use std::collections::HashMap;

enum Error {
    FunctionNotFound,
}

#[derive(Copy, Clone)]
struct Function<'a> {
    name: &'a str,
    code: &'a [u32],
}

struct Context<'a> {
    program: HashMap<&'a str, Function<'a>>,
    call_stack: Vec<Function<'a>>,
}

impl<'a> Context<'a> {
    fn get_function(&mut self, fun_name: &str) -> Result<Function<'a>, Error> {
        self.program
            .get(fun_name)
            .map(|f| *f)
            .ok_or(Error::FunctionNotFound)
    }

    fn call(&mut self, fun_name: &str) -> Result<(), Error> {
        let fun = self.get_function(fun_name)?;

        self.call_stack.push(fun);

        Ok(())
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut context = Context {
            program: HashMap::new(),
            call_stack: Vec::new(),
        };

        let fun = Function {
            name: "main",
            code: &[1, 2, 3],
        };

        context.program.insert("main", fun);

        let res = context.call("main");
        assert!(res.is_ok());
    }
}
