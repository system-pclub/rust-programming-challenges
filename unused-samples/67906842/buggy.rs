use std::{collections::HashMap, rc::Rc};

struct Context<'a> {
    parent: Option<&'a mut Context<'a>>,
    symbols: HashMap<String, i64>,
}

impl<'a> Context<'a> {
    fn new() -> Self {
        Context {
            parent: None,
            symbols: HashMap::new(),
        }
    }

    fn derive(&'a mut self) -> Self {
        Context {
            parent: Some(self),
            symbols: HashMap::new(),
        }
    }
}

#[derive(Clone)]
struct Statement {
    execute: Rc<dyn Fn(&mut Context) -> Result<()>>,
}

struct Node {
    cond: Statement,
    stmts: Vec<Statement>,
}

impl Node {
    fn get(&self) -> Statement {
        let cond = self.cond.clone();
        let stmts = self.stmts.clone();
        Statement {
            execute: Rc::new(move |ctx| {
                (cond.execute)(ctx)?;
                let mut cctx = ctx.derive();
                for stmt in stmts {
                    (stmt.execute)(&mut cctx)?;
                }
                Ok(())
            }),
        }
    }
}