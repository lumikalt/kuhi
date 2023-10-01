use std::fmt::Display;

use crate::{
    builtins::BUILTINS,
    err::RuntimeError,
    parser::{Loc, Token},
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

    pub fn repurpose(&mut self, tokens: &'a Vec<(Token, Loc)>) -> &mut Self {
        self.tokens = tokens;

        self
    }

    pub fn run(&mut self) -> Result<(), (RuntimeError, Loc)> {
        for (token, loc) in self.tokens.iter().rev() {
            let stack = &mut self.stack;
            match token {
                Token::Integer(n) => stack.push(Value::Integer(n.clone())),
                Token::Rational(r) => stack.push(Value::Rational(r.clone())),
                Token::Complex(c) => stack.push(Value::Complex(c.clone())),

                Token::Function(_f) => {
                    todo!("Implement function application")
                }
                Token::Add => {
                    match unsafe { BUILTINS.get(&'+').unwrap_unchecked() }.call(stack.clone()) {
                        Err(err) => return Err((err, loc.clone())),
                        Ok(stack) => {
                            self.stack = stack;
                        }
                    }
                }
                Token::Subtract => {
                    match unsafe { BUILTINS.get(&'-').unwrap_unchecked() }.call(stack.clone()) {
                        Err(err) => return Err((err, loc.clone())),
                        Ok(stack) => {
                            self.stack = stack;
                        }
                    }
                }
                Token::Multiply => {
                    match unsafe { BUILTINS.get(&'×').unwrap_unchecked() }.call(stack.clone()) {
                        Err(err) => return Err((err, loc.clone())),
                        Ok(stack) => {
                            self.stack = stack;
                        }
                    }
                }
                Token::Divide => {
                    match unsafe { BUILTINS.get(&'÷').unwrap_unchecked() }.call(stack.clone()) {
                        Err(err) => return Err((err, loc.clone())),
                        Ok(stack) => {
                            self.stack = stack;
                        }
                    }
                }
                Token::Power => {
                    match unsafe { BUILTINS.get(&'ⁿ').unwrap_unchecked() }.call(stack.clone()) {
                        Err(err) => return Err((err, loc.clone())),
                        Ok(stack) => {
                            self.stack = stack;
                        }
                    }
                }
                Token::Factorial => {
                    match unsafe { BUILTINS.get(&'!').unwrap_unchecked() }.call(stack.clone()) {
                        Err(err) => return Err((err, loc.clone())),
                        Ok(stack) => {
                            self.stack = stack;
                        }
                    }
                }
                Token::Modulo => {
                    match unsafe { BUILTINS.get(&'◿').unwrap_unchecked() }.call(stack.clone()) {
                        Err(err) => return Err((err, loc.clone())),
                        Ok(stack) => {
                            self.stack = stack;
                        }
                    }
                }

                _ => todo!("Implement other tokens"),
            }
        }

        Ok(())
    }
}

impl Display for Env<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stack = self.stack.clone();

        let stack = stack
            .into_iter()
            .rev()
            .enumerate()
            .fold("".to_string(), |acc, v| {
                acc + &format!("[{}] {} \n", v.0 + 1, v.1)
            });

        write!(f, "{}", stack.trim())
    }
}
