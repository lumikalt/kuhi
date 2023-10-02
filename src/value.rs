use std::{fmt::Display, ops::*};

use rug::{ops::Pow, Complex, Integer, Rational};

use crate::{
    builtins::{RuntimeResult, Stack},
    err::RuntimeError,
    parser::Token,
};

#[derive(Debug, Clone)]
pub enum Value {
    Integer(Integer),
    Rational(Rational),
    Complex(Complex),
    Symbol(String),
    List(Vec<Value>),
    Function(fn(&mut Stack) -> RuntimeResult),

    InvalidState(RuntimeError),
}

impl Value {
    pub fn type_id(&self) -> String {
        match self {
            Value::Integer(_) => "Integer".to_string(),
            Value::Rational(_) => "Rational".to_string(),
            Value::Complex(_) => "Complex".to_string(),
            Value::Symbol(_) => "Symbol".to_string(),
            Value::List(_) => "List".to_string(),
            Value::Function(_) => "Function".to_string(),

            Value::InvalidState(_) => unreachable!("InvalidState should never be used"),
        }
    }

    pub fn reciprocal(&self) -> Self {
        match self {
            Value::Integer(n) => Value::Rational(Rational::from((1, n.clone()))),
            Value::Rational(r) => Value::Rational(r.clone().recip()),
            Value::Complex(c) => Value::Complex(c.clone().recip()),

            invalid => Value::InvalidState(RuntimeError::TypeMissmatch {
                expected: "Numeric".to_string(),
                got: format!("{}", invalid.type_id()),
            }),
        }
    }

    pub fn pow(&self, rhs: &Self) -> Self {
        match (self, rhs) {
            (Value::Integer(n), Value::Integer(m)) => {
                if let Some(m) = m.to_u32() {
                    Value::Integer(n.clone().pow(m))
                } else {
                    Value::InvalidState(RuntimeError::ExponentTooBig(m.clone()))
                }
            }
            (Value::Rational(r), Value::Integer(n)) => {
                if let Some(n) = n.to_u32() {
                    Value::Rational(r.clone().pow(n))
                } else {
                    Value::InvalidState(RuntimeError::ExponentTooBig(n.clone()))
                }
            }
            _ => {
                todo!()
            }
        }
    }

    pub fn root(&self, rhs: &Self) -> Self {
        match (self, rhs) {
            (Value::Integer(n), Value::Integer(m)) => {
                Value::Integer(n.clone().root(m.to_u32_wrapping()))
            }
            _ => {
                todo!()
            }
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Value::*;

        match self {
            Integer(n) => write!(f, "{}", n),
            Rational(r) => write!(f, "{}", r),
            Complex(c) => write!(f, "{}+{}i", c.real().to_f64(), c.imag().to_f64()),
            Symbol(s) => write!(f, "{}", s),
            List(l) => {
                write!(f, "(")?;
                for v in l {
                    write!(f, "{} ", v)?;
                }
                write!(f, ")")
            }
            Function(_) => write!(f, "<function>"),

            InvalidState(err) => write!(f, "{}", err),
        }
    }
}

impl From<Token> for Value {
    fn from(token: Token) -> Self {
        match token {
            Token::Integer(n) => Value::Integer(n),
            Token::Rational(r) => Value::Rational(r),
            Token::Complex(c) => Value::Complex(c),
            // Token::List(tokens) => Value::List(tokens.into_iter().map(Value::from).collect()),
            // Token::Function(f) => Value::Function(f),
            _ => {
                todo!()
            }
        }
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        use Value::*;

        match (self, rhs) {
            (Integer(n), Integer(m)) => Integer(n + m),
            (Rational(r), Rational(s)) => Rational(r + s),
            (Complex(c), Complex(d)) => Complex(c + d),
            (Integer(n), Rational(r)) => Rational(n + r),
            (Rational(r), Integer(n)) => Rational(n + r),
            (Integer(n), Complex(c)) => Complex(n + c),
            (Complex(c), Integer(n)) => Complex(n + c),
            (Rational(r), Complex(c)) => Complex(c + r),
            (Complex(c), Rational(r)) => Complex(c + r),
            _ => {
                todo!()
            }
        }
    }
}

impl Neg for Value {
    type Output = Self;

    fn neg(self) -> Self::Output {
        use Value::*;

        match self {
            Integer(n) => Integer(-n),
            Rational(r) => Rational(-r),
            Complex(c) => Complex(-c),

            invalid => InvalidState(RuntimeError::TypeMissmatch {
                expected: "Numeric".to_string(),
                got: format!("{}", invalid.type_id()),
            }),
        }
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        use Value::*;

        match (self, rhs) {
            (Integer(n), Integer(m)) => Integer(n * m),
            (Rational(r), Rational(s)) => Rational(r * s),
            (Complex(c), Complex(d)) => Complex(c * d),
            (Integer(n), Rational(r)) => Rational(n * r),
            (Rational(r), Integer(n)) => Rational(n * r),
            (Integer(n), Complex(c)) => Complex(n * c),
            (Complex(c), Integer(n)) => Complex(n * c),
            (Rational(r), Complex(c)) => Complex(c * r),
            (Complex(c), Rational(r)) => Complex(c * r),
            _ => {
                todo!()
            }
        }
    }
}
