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

                Token::List(vals) => stack.push(Value::List(
                    vals.into_iter()
                        .map(|tok| tok.clone().into())
                        .collect::<Vec<_>>(),
                )),
                Token::_UnfinishedList(_) => unreachable!("Unfinished list"),

                Token::Scope(op) => {
                    let top = if let Some(value) = stack.pop() {
                        value
                    } else {
                        return Err((
                            RuntimeError::InvalidPop {
                                len: stack.len(),
                                arity: 1,
                            },
                            loc.clone(),
                        ));
                    };

                    match top {
                        Value::List(vals) => {
                            let mut inner_env = Env::new(op.to_vec());
                            inner_env.stack = vals;
                            inner_env.run()?;

                            match inner_env.stack.len() {
                                0 => continue,
                                1 => stack.push(inner_env.stack.pop().unwrap()),
                                _ => stack.push(Value::List(inner_env.stack)),
                            }
                        }
                        _ => todo!("Implement scope for non-list values"),
                    }
                }

                Token::Infinity => stack.push(Value::Infinity(1)),
                Token::Epsilon => stack.push(Value::Epsilon(1)),
                Token::Pi(r) => stack.push(Value::Pi(r.clone(), 1)),
                Token::E(r, pow) => stack.push(Value::E(r.clone(), *pow)),

                Token::Dup => {
                    if let Some(value) = stack.pop() {
                        stack.push(value.clone());
                        stack.push(value);
                    } else {
                        return Err((
                            RuntimeError::InvalidPop {
                                len: stack.len(),
                                arity: 1,
                            },
                            loc.clone(),
                        ));
                    }
                }
                Token::Pop => {
                    if let Some(_) = stack.pop() {
                        // Do nothing
                    } else {
                        return Err((
                            RuntimeError::InvalidPop {
                                len: stack.len(),
                                arity: 1,
                            },
                            loc.clone(),
                        ));
                    }
                }
                Token::Flip => {
                    match unsafe { BUILTINS.get(&'↔').unwrap_unchecked() }.call(stack.clone()) {
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
                        return Err((
                            RuntimeError::InvalidPop {
                                len: stack.len(),
                                arity: 1,
                            },
                            loc.clone(),
                        ));
                    }
                }

                Token::FunctionCall(c) => match BUILTINS.get(&c) {
                    Some(builtin) => match builtin.call(stack.clone()) {
                        Err(err) => return Err((err, loc.clone())),
                        Ok(stack) => {
                            self.stack = stack;
                        }
                    },
                    None => return Err((RuntimeError::FunctionNotFound(c.clone()), loc.clone())),
                },

                Token::Function(_f) => {
                    todo!("Implement function application")
                }
                Token::Inverse(tok, loc) => match tok.as_ref() {
                    Token::FunctionCall(c) => match BUILTINS.get(&c) {
                        Some(builtin) => match builtin.call_inverse(stack.clone()) {
                            Err(err) => return Err((err, loc.clone())),
                            Ok(stack) => {
                                self.stack = stack;
                            }
                        },
                        None => {
                            return Err((RuntimeError::FunctionNotFound(c.clone()), loc.clone()))
                        }
                    },
                    _ => Err((RuntimeError::InverseOfNonFunction, dbg!(loc.clone())))?,
                },
                Token::InvalidState => unreachable!(),
            }

            if let Value::InvalidState(err) = self.stack.last().unwrap_or(&Value::Epsilon(0)) {
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
