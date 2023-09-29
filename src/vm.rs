use crate::{
    builtins,
    err::{Loc, RuntimeError},
    parser::Token,
    value::Value,
};

/// Runtime

pub struct Env<'a> {
    pub stack: Vec<Value>,

    tokens: &'a Vec<(Token, Loc)>,
}

impl<'a> Env<'a> {
    pub fn new(tokens: &'a Vec<(Token, Loc)>) -> Self {
        Self {
            stack: Vec::new(),
            tokens,
        }
    }

    pub fn run(&mut self) -> Result<(), (RuntimeError, Loc)> {
        for (token, loc) in self.tokens.iter().rev() {
            let stack = &mut self.stack;
            match token {
                Token::Number(n) => stack.push(Value::Number(n.clone())),
                Token::Rational(r) => stack.push(Value::Rational(r.clone())),
                Token::Function(_f) => {
                    todo!("Implement function application")
                }
                Token::Plus => match builtins::BUILTINS.get(&'+').unwrap().call(stack.clone()) {
                    Err(err) => return Err((err, loc.clone())),
                    Ok(stack) => {
                        self.stack = stack;
                    }
                },
                _ => todo!("Implement other tokens"),
            }
        }

        Ok(())
    }
}
