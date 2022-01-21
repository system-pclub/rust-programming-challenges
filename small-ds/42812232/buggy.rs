use std::collections::HashMap;
use std::cell::RefCell;
#[derive(Clone, Debug)]
pub enum Value<'a> {
    Number(f64),
    UserFunc(Definition, Enviroment<'a>),
}

#[derive(Clone, Debug)]
pub struct Definition;

pub enum Expr {
    Name(String),
}

// Nothing to do with the problem
pub enum Error {
    UndefinedName(String),
}

#[derive(Debug, Clone)]
pub struct Enviroment<'a> {
    current_frame: HashMap<String, Option<Value<'a>>>,
    prev: Option<&'a Enviroment<'a>>,
}
impl<'a> Enviroment<'a> {
    pub fn new() -> Enviroment<'a> {
        Enviroment {
            current_frame: HashMap::new(),
            prev: None,
        }
    }
    pub fn extend(bindings: Vec<(String, Value<'a>)>,
                  prev: Option<&'a Enviroment<'a>>)
                  -> Enviroment<'a> {
        let mut frame = HashMap::new();
        for (key, val) in bindings {
            frame.insert(key, Some(val));
        }
        Enviroment {
            current_frame: frame,
            prev: prev,
        }
    }
    pub fn lookup(&self, name: &str) -> Option<Option<Value>> {
        let val = self.current_frame.get(&String::from(name));
        if val.is_some() {
            val.cloned()
        } else {
            if let Some(prev) = self.prev {
                prev.lookup(name)
            } else {
                None
            }
        }
    }
}

type RefEnv<'a> = RefCell<Enviroment<'a>>;

fn eval_expr<'a, 'b>(ast: &'a Expr, env: &'b RefEnv<'b>) -> Result<Value<'b>, Error> {
    match *ast {
        Expr::Name(ref name) => {
            let e = env.borrow();
            let val = e.lookup(&name);
            if let Some(Some(v)) = val {
                Ok(v)
            } else {
                Err(Error::UndefinedName(format!("{} is not defined", name)))
            }
        }
    }
}

fn main() {}