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
    pub fn lookup<'b, 'c>(&'b self, name: &'c str) -> Option<Option<Value<'a>>> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_expr_undefined_variable() {
        let env = RefCell::new(Enviroment::new());
        let ast = Expr::Name("x".to_string());
        let res = eval_expr(&ast, &env);
        assert!(matches!(res, Err(Error::UndefinedName(_))));
    }

    #[test]
    fn test_eval_expr_defined_variable() {
        let mut env = RefCell::new(Enviroment::new());
        env.borrow_mut().current_frame.insert("x".to_string(), Some(Value::Number(42.0)));

        let ast = Expr::Name("x".to_string());
        let res = eval_expr(&ast, &env);
        assert!(matches!(res, Ok(Value::Number(42.0))));
    }

    #[test]
    fn test_eval_expr_extended_environment() {
        let base_env = Enviroment::new();
        let extended_env = Enviroment::extend(vec![("y".to_string(), Value::Number(84.0))], Some(&base_env));
        let env = RefCell::new(extended_env);

        let ast = Expr::Name("y".to_string());
        let res = eval_expr(&ast, &env);
        assert!(matches!(res, Ok(Value::Number(84.0))));
    }

    #[test]
    fn test_eval_expr_nested_environment() {
        let base_env = Enviroment::extend(vec![("x".to_string(), Value::Number(42.0))], None);
        let extended_env = Enviroment::extend(vec![("y".to_string(), Value::Number(84.0))], Some(&base_env));
        let env = RefCell::new(extended_env);

        let ast_x = Expr::Name("x".to_string());
        let res_x = eval_expr(&ast_x, &env);
        assert!(matches!(res_x, Ok(Value::Number(42.0))));

        let ast_y = Expr::Name("y".to_string());
        let res_y = eval_expr(&ast_y, &env);
        assert!(matches!(res_y, Ok(Value::Number(84.0))));
    }

    #[test]
    fn test_eval_expr_variable_shadowing() {
        let base_env = Enviroment::extend(vec![("x".to_string(), Value::Number(42.0))], None);
        let extended_env = Enviroment::extend(vec![("x".to_string(), Value::Number(84.0))], Some(&base_env));
        let env = RefCell::new(extended_env);

        let ast = Expr::Name("x".to_string());
        let res = eval_expr(&ast, &env);
        assert!(matches!(res, Ok(Value::Number(84.0))));
    }
}

