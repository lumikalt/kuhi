use std::{fmt::Display, ops::*};

use rug::{float::Constant as consts, ops::Pow, Complex, Float, Integer, Rational};

use lazy_static::lazy_static;

use crate::{
    builtins::{RuntimeResult, Stack},
    err::RuntimeError,
    parser::Token,
};

lazy_static! {
    pub static ref PI: Float = Float::with_val(128, consts::Pi);
    pub static ref E: Float = Float::with_val(128, Float::exp(Float::with_val(128, 1)));
}

#[derive(Debug, Clone)]
pub enum Value {
    Integer(Integer),
    Rational(Rational),
    Complex(Complex),
    Float(Float),

    List(Vec<Value>),
    Function(fn(&mut Stack) -> RuntimeResult),

    // Math specials
    Infinity(i8),
    Undefined,
    Pi(Rational, i8), // i8 for exponent sign (1 for pi, -1 for pi^-1)
    E(Rational, i32),
    Epsilon(i8), // Very small number, such that 1/Epsilon == +Infinity

    InvalidState(RuntimeError),
}

impl Value {
    pub fn types(&self) -> Vec<&str> {
        match self {
            Value::Integer(_) => vec!["Number", "Integer"],
            Value::Rational(_) => vec!["Number", "Rational"],
            Value::Complex(_) => vec!["Number", "Complex"],
            Value::Float(_) => vec!["Number", "Float"],

            Value::List(_) => vec!["List"],
            Value::Function(_) => vec!["Function"],

            Value::Infinity(_) => vec!["Number", "Infinity"],
            Value::Undefined => vec!["Number", "Undefined"],
            Value::Pi(_, _) => vec!["Number", "Pi"],
            Value::E(_, _) => vec!["Number", "E"],
            Value::Epsilon(_) => vec!["Number", "Epsilon"],

            Value::InvalidState(_) => unreachable!("InvalidState should never be used"),
        }
    }

    pub fn reciprocal(&self) -> Self {
        if !self.types().contains(&"Number") {
            return Value::InvalidState(RuntimeError::TypeMissmatch {
                expected: "Number".to_string(),
                got: format!("{}", self.types().join(", ")),
            });
        }
        if self.is_zero() {
            return Value::InvalidState(RuntimeError::DivideByZero);
        }

        match self {
            Value::Integer(n) => Value::Rational(Rational::from((1, n.clone()))),
            Value::Rational(r) => Value::Rational(r.clone().recip()),
            Value::Complex(z) => Value::Complex(z.clone().recip()),
            Value::Float(x) => Value::Float(x.clone().recip()),
            Value::Infinity(sign) => Value::Epsilon(*sign),
            Value::Epsilon(sign) => Value::Infinity(*sign),
            Value::Pi(r, esign) => Value::Pi(r.clone().recip(), -*esign),
            Value::E(r, exp) => Value::E(r.clone().recip(), -*exp),

            invalid => Value::InvalidState(RuntimeError::TypeMissmatch {
                expected: "Numeric".to_string(),
                got: format!(
                    "{}",
                    invalid
                        .types()
                        .into_iter()
                        .rev()
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            }),
        }
    }

    pub fn pow(&self, rhs: &Self) -> Self {
        if !self.types().contains(&"Number") || !rhs.types().contains(&"Number") {
            return Value::InvalidState(RuntimeError::TypeMissmatch {
                expected: "Number".to_string(),
                got: format!("{} and {}", self.types().join(", "), rhs.types().join(", ")),
            });
        }

        match (self, rhs) {
            (Value::Integer(n), Value::Integer(m)) => {
                if let Some(m) = m.to_u32() {
                    Value::Integer(n.clone().pow(m))
                } else {
                    Value::InvalidState(RuntimeError::ExponentTooBig(m.clone()))
                }
            }
            (Value::Rational(r), Value::Integer(n)) => {
                if let Some(n) = n.to_i32() {
                    Value::Rational(r.clone().pow(n))
                } else {
                    if let Some(n) = n.to_u32() {
                        Value::Rational(r.clone().pow(n))
                    } else {
                        Value::InvalidState(RuntimeError::ExponentTooBig(n.clone()))
                    }
                }
            }
            _ => {
                todo!()
            }
        }
    }

    pub fn root(&self, rhs: &Self) -> Self {
        if !self.types().contains(&"Number") || !rhs.types().contains(&"Number") {
            return Value::InvalidState(RuntimeError::TypeMissmatch {
                expected: "Number".to_string(),
                got: format!("{} and {}", self.types().join(", "), rhs.types().join(", ")),
            });
        }

        match (self, rhs) {
            (Value::Integer(n), Value::Integer(m)) => {
                Value::Integer(n.clone().root(m.to_u32_wrapping()))
            }
            _ => {
                todo!()
            }
        }
    }

    pub fn is_zero(&self) -> bool {
        match self {
            Value::Integer(n) => n.is_zero(),
            Value::Rational(r) => r.is_zero(),
            Value::Complex(c) => c.is_zero(),
            Value::Float(x) => x.is_zero(),

            Value::Infinity(_) => false,
            Value::Undefined => false,
            Value::Pi(r, _) => r.is_zero(),
            Value::E(r, _) => r.is_zero(),
            Value::Epsilon(_) => false,

            Value::InvalidState(_) => unreachable!("InvalidState should never be used"),

            _ => {
                todo!()
            }
        }
    }
}

impl From<Token> for Value {
    fn from(token: Token) -> Self {
        match token {
            Token::Integer(n) => Value::Integer(n),
            Token::Rational(r) => Value::Rational(r),
            Token::Complex(c) => Value::Complex(c),
            Token::Pi(r) => Value::Pi(r, 1),
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

        if !self.types().contains(&"Number") || !rhs.types().contains(&"Number") {
            return InvalidState(RuntimeError::TypeMissmatch {
                expected: "Number".to_string(),
                got: format!("{} and {}", self.types().join(", "), rhs.types().join(", ")),
            });
        }

        match (self, rhs) {
            (Integer(n), Integer(m)) => Integer(n + m),
            (Rational(r), Rational(s)) => Rational(r + s),
            (Complex(c), Complex(d)) => Complex(c + d),
            (Integer(n), Rational(r)) | (Rational(r), Integer(n)) => Rational(n + r),
            (Integer(n), Complex(c)) | (Complex(c), Integer(n)) => Complex(n + c),
            (Rational(r), Complex(c)) | (Complex(c), Rational(r)) => Complex(c + r),
            (Float(x), Float(y)) => Float(x + y),
            (Float(x), Integer(n)) | (Integer(n), Float(x)) => Float(x + n),
            (Float(x), Rational(r)) | (Rational(r), Float(x)) => Float(
                x + rug::Float::with_val(128, r.numer()) / rug::Float::with_val(128, r.denom()),
            ),

            (Infinity(a), Infinity(b)) => {
                if a == b {
                    Infinity(a)
                } else {
                    Undefined
                }
            }
            (Infinity(a), Integer(_))
            | (Integer(_), Infinity(a))
            | (Infinity(a), Rational(_))
            | (Rational(_), Infinity(a))
            | (Infinity(a), Float(_))
            | (Float(_), Infinity(a)) => Infinity(a),

            (Infinity(_), Complex(_)) | (Complex(_), Infinity(_)) => {
                todo!("Implement Infinity + Complex (study complex analysis)")
            }

            (Epsilon(a), Epsilon(b)) => {
                if a.signum() == b.signum() {
                    Epsilon(a)
                } else {
                    Undefined
                }
            }
            (Epsilon(_), anything) | (anything, Epsilon(_)) => anything,

            (Undefined, _) | (_, Undefined) => Undefined,

            // Can't keep Pi unevaluated, so we approximate it
            (Pi(r, esign), Integer(n)) | (Integer(n), Pi(r, esign)) => Float(
                (rug::Float::with_val(128, r.numer()) / rug::Float::with_val(128, r.denom()))
                    * match esign {
                        1 => PI.clone(),
                        -1 => PI.clone().recip(),
                        _ => unreachable!(),
                    }
                    + rug::Float::with_val(128, n),
            ),
            (Pi(r, esign), Rational(s)) | (Rational(s), Pi(r, esign)) => Float(
                (rug::Float::with_val(128, r.numer()) / rug::Float::with_val(128, r.denom()))
                    * match esign {
                        1 => PI.clone(),
                        -1 => PI.clone().recip(),
                        _ => unreachable!(),
                    }
                    + rug::Float::with_val(128, s.numer()) / rug::Float::with_val(128, s.denom()),
            ),
            (Pi(r, esign), Complex(c)) | (Complex(c), Pi(r, esign)) => Complex(
                (rug::Float::with_val(128, r.numer()) / rug::Float::with_val(128, r.denom()))
                    * match esign {
                        1 => PI.clone(),
                        -1 => PI.clone().recip(),
                        _ => unreachable!(),
                    }
                    + c,
            ),

            // (E(r, exp), Integer(n)) | (Integer(n), E(r, exp)) => Rational(n * r),
            // (E(r, exp), Rational(s)) | (Rational(s), E(r, exp)) => Rational(s * r),
            // (E(r, exp), Complex(c)) | (Complex(c), E(r, exp)) => Complex(c * r * E),
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
            Float(x) => Float(-x),
            Complex(c) => Complex(-c),

            Infinity(sign) => Infinity(-sign),
            Epsilon(sign) => Epsilon(-sign),
            Pi(r, esign) => Pi(-r, esign),
            E(r, exp) => E(-r, exp),

            invalid => InvalidState(RuntimeError::TypeMissmatch {
                expected: "Numeric".to_string(),
                got: format!(
                    "{}",
                    invalid
                        .types()
                        .into_iter()
                        .rev()
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
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
            (Float(x), Float(y)) => Float(x * y),
            (Complex(c), Complex(d)) => Complex(c * d),
            (Integer(n), Rational(r)) | (Rational(r), Integer(n)) => Rational(n * r),
            (Integer(n), Complex(c)) | (Complex(c), Integer(n)) => Complex(n * c),
            (Rational(r), Complex(c)) | (Complex(c), Rational(r)) => Complex(c * r),
            (Float(x), Integer(n)) | (Integer(n), Float(x)) => Float(x * n),
            (Float(x), Rational(r)) | (Rational(r), Float(x)) => Float(
                x * rug::Float::with_val(128, r.numer()) / rug::Float::with_val(128, r.denom()),
            ),
            (Float(x), Complex(c)) | (Complex(c), Float(x)) => Complex(x * c),

            (Infinity(a), Infinity(b)) => Infinity(a * b),
            (Infinity(a), Integer(n)) | (Integer(n), Infinity(a)) => match n {
                n if n.is_positive() => Infinity(a),
                n if n.is_negative() => Infinity(-a),
                _ => Undefined,
            },
            (Infinity(a), Rational(r)) | (Rational(r), Infinity(a)) => match r {
                r if r.is_positive() => Infinity(a),
                r if r.is_negative() => Infinity(-a),
                _ => Undefined,
            },
            (Infinity(a), Float(x)) | (Float(x), Infinity(a)) => {
                if x.is_zero() {
                    Undefined
                } else {
                    Infinity(a * x.signum().to_f32() as i8)
                }
            }
            (Infinity(_), Complex(_)) | (Complex(_), Infinity(_)) => {
                todo!("Implement Infinity + Complex (study complex analysis)")
            }

            (Epsilon(a), Epsilon(b)) => Epsilon(a * b),
            (Infinity(_), Epsilon(_)) | (Epsilon(_), Infinity(_)) => Undefined,

            (Epsilon(a), Integer(n)) | (Integer(n), Epsilon(a)) => Integer(n * a),
            (Epsilon(a), Rational(r)) | (Rational(r), Epsilon(a)) => Rational(r * a),
            (Epsilon(a), Float(x)) | (Float(x), Epsilon(a)) => Float(x * a),
            (Epsilon(a), Complex(z)) | (Complex(z), Epsilon(a)) => Complex(z * a),

            (Undefined, _) | (_, Undefined) => Undefined,

            (Pi(r, esign), Integer(n)) | (Integer(n), Pi(r, esign)) => Pi(r.clone() * n, esign),
            (Pi(r, esign), Rational(s)) | (Rational(s), Pi(r, esign)) => Pi(r.clone() * s, esign),
            (Pi(r, esign), Complex(c)) | (Complex(c), Pi(r, esign)) => Complex(
                (rug::Float::with_val(128, r.numer()) / rug::Float::with_val(128, r.denom()))
                    * match esign {
                        1 => PI.clone(),
                        -1 => PI.clone().recip(),
                        _ => unreachable!(),
                    }
                    * c,
            ),

            // (E(r, exp), Integer(n)) | (Integer(n), E(r, exp)) => Rational(n * r),
            // (E(r, exp), Rational(s)) | (Rational(s), E(r, exp)) => Rational(s * r),
            // (E(r, exp), Complex(c)) | (Complex(c), E(r, exp)) => Complex(c * r * E),
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
            Integer(n) => write!(f, "{}", n.to_string().replace('-', "⁻")),
            Rational(r) => write!(f, "{}", r.to_string().replace('-', "⁻")),
            Complex(z) => write!(
                f,
                "{}i{}",
                z.real().to_f64().to_string().replace('-', "⁻"),
                z.imag().to_f64().to_string().replace('-', "⁻")
            ),
            Float(x) => write!(f, "{}", x.to_f64().to_string().replace('-', "⁻")),

            List(l) => {
                write!(f, "(")?;
                for v in l {
                    write!(f, "{} ", v)?;
                }
                write!(f, ")")
            }
            Function(_) => write!(f, "<function>"),

            Infinity(sign) => {
                write!(
                    f,
                    "{}∞",
                    match sign.signum() {
                        1 => "+",
                        -1 => "⁻",
                        _ => unreachable!(),
                    }
                )
            }
            Epsilon(sign) => {
                write!(
                    f,
                    "{}ε",
                    match sign.signum() {
                        1 => "+",
                        -1 => "⁻",
                        _ => unreachable!(),
                    }
                )
            }
            Undefined => write!(f, "undef"),
            Pi(r, esign) => write!(
                f,
                "{}{}{}",
                r.numer().to_string().replace('-', "⁻"),
                match esign {
                    1 => "π/",
                    -1 => "/π",
                    _ => unreachable!(),
                },
                r.denom().to_string().replace('-', "⁻")
            ),
            E(r, exp) => write!(
                f,
                "{}e^{}/{}",
                r.numer().to_string().replace('-', "⁻"),
                exp,
                r.denom().to_string().replace('-', "⁻")
            ),

            InvalidState(err) => write!(f, "{}", err),
        }
    }
}
