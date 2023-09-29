use std::ops::Add;

use rug::{Complex, Integer, Rational};

use crate::{parser::Token, builtins::{Stack, RuntimeResult}};

#[derive(Debug, Clone)]
pub enum Value {
    Number(Integer),
    Rational(Rational),
    Complex(Complex),
    Symbol(String),
    List(Vec<Value>),
    Function(fn(&mut Stack) -> RuntimeResult),
}

impl From<Token> for Value {
    fn from(token: Token) -> Self {
        match token {
            Token::Number(n) => Value::Number(n),
            Token::Rational(r) => Value::Rational(r),
            // Token::Complex(c) => Value::Complex(c),
            // Token::List(tokens) => Value::List(tokens.into_iter().map(Value::from).collect()),
            // Token::Function(f) => Value::Function(f),
            _ => {
                todo!()
            }
        }
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        use Value::*;
        match (self, rhs) {
            (Number(n), Number(m)) => Number(n + m),
            (Rational(r), Rational(s)) => Rational(r + s),
            (Complex(c), Complex(d)) => Complex(c + d),
            (Number(n), Rational(r)) => Rational(n + r),
            (Rational(r), Number(n)) => Rational(n + r),
            (Number(n), Complex(c)) => Complex(n + c),
            (Complex(c), Number(n)) => Complex(n + c),
            (Rational(r), Complex(c)) => Complex(c + r),
            (Complex(c), Rational(r)) => Complex(c + r),
            _ => {
                todo!()
            }
        }
    }
}
