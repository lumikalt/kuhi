use std::fmt::Display;

use crate::{
    builtins::BUILTINS,
    err::RuntimeError,
    parser::{Loc, Token},
    value::Value,
};

/// Runtime

pub struct Env {
    pub stack: Vec<Value>,

    tokens: Vec<(Token, Loc)>,
}

impl Env {
    pub fn new(tokens: Vec<(Token, Loc)>) -> Self {
        Self {
            stack: Vec::new(),
            tokens,
        }
    }

    pub fn repurpose(&mut self, tokens: &Vec<(Token, Loc)>) -> &mut Self {
        self.tokens = tokens.clone();

        self
    }

    pub fn run(&mut self) -> Result<(), (RuntimeError, Loc)> {
        for (token, loc) in self.tokens.iter().rev() {
            let stack = &mut self.stack;
            match token {
                Token::Spacing => continue,
                Token::Integer(n) => stack.push(Value::Integer(n.clone())),
                Token::Rational(r) => stack.push(Value::Rational(r.clone())),
                Token::Complex(c) => stack.push(Value::Complex(c.clone())),

                Token::Infinity => stack.push(Value::Infinity(1)),
                Token::Epsilon => stack.push(Value::Epsilon(1)),
                Token::Pi(r) => stack.push(Value::Pi(r.clone(), 1)),
                Token::E(r, pow) => stack.push(Value::E(r.clone(), *pow)),

                Token::Dup => {
                    if let Some(value) = stack.pop() {
                        stack.push(value.clone());
                        stack.push(value);
                    } else {
                        return Err((RuntimeError::InvalidPop { len: stack.len(), arity: 1 }, loc.clone()));
                    }
                }
                Token::Pop => {
                    if let Some(_) = stack.pop() {
                        // Do nothing
                    } else {
                        return Err((RuntimeError::InvalidPop { len: stack.len(), arity: 1 }, loc.clone()));
                    }
                }
                Token::Flip => { match unsafe { BUILTINS.get(&'.').unwrap_unchecked() }.call(stack.clone()) {
                        Err(err) => return Err((err, loc.clone())),
                        Ok(stack) => {
                            self.stack = stack;
                        }
                    }
                }
                Token::Minus => {
                    if let Some(value) = stack.pop() {
                        stack.push(-value);
                    } else {
                        return Err((RuntimeError::InvalidPop { len: stack.len(), arity: 1 }, loc.clone()));
                    }
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
                Token::Root => {
                    match unsafe { BUILTINS.get(&'√').unwrap_unchecked() }.call(stack.clone()) {
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

                Token::Function(_f) => {
                    todo!("Implement function application")
                }
                Token::Inverse => todo!("Inverse"),

                Token::InvalidState => unreachable!(),
            }

            if let Value::InvalidState(err) = self.stack.last().unwrap() {
                return Err((err.clone(), loc.clone()));
            }
        }

        Ok(())
    }
}

impl Display for Env {
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
