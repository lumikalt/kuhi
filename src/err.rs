use std::fmt::{self, Display, Formatter};

use rug::Integer;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SyntaxError {
    InvalidSymbol(char),
    /// true if `(`, false if `)`
    UnmatchedParenthesis(bool),
    LonelyInverse,
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SyntaxError::InvalidSymbol(_) => write!(f, "invalid symbol"),
            SyntaxError::UnmatchedParenthesis(_) => write!(f, "unmatched parenthesis"),
            SyntaxError::LonelyInverse => write!(f, "lonely inverse"),
        }
    }
}

impl SyntaxError {
    pub fn note(&self) -> String {
        match self {
            SyntaxError::InvalidSymbol(_) => {
                "check the docs for a list of valid symbols".to_owned()
            }
            SyntaxError::UnmatchedParenthesis(open) => format!(
                "there is a missing {} parenthesis in the code",
                if *open { "opening" } else { "closing" }
            ),
            SyntaxError::LonelyInverse => "must have something to invert".to_owned(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum RuntimeError {
    FunctionNotFound(char),

    ListTypeMissmatch { first: String, second: String },
    ListElementSizeMissmatch { first: usize, second: usize },

    InvalidPop { len: usize, arity: usize },

    InvalidFoldWith(usize),
    InvalidMapWith(usize),
    InvalidFilterWith(usize),

    TypeMissmatch { expected: String, got: String },

    ExponentTooBig(Integer),
    ZerothRoot,
    DivideByZero,

    NoInverse,
    InverseOfNonFunction,
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeError::FunctionNotFound(c) => write!(f, "function `{}` not found", c),
            RuntimeError::ListTypeMissmatch { first, second } => write!(
                f,
                "list has an element of type `{}` followed by one of type `{}`",
                first, second
            ),
            RuntimeError::ListElementSizeMissmatch { first, second } => write!(
                f,
                "list has an element of size `{}` followed by one of size `{}`",
                first, second
            ),
            RuntimeError::InvalidPop { len, arity } => write!(
                f,
                "attempt to pop {} times from a stack of size {}",
                arity, len
            ),
            RuntimeError::InvalidFoldWith(arity) => {
                write!(f, "attempt to fold using a function of arity {}", arity)
            }
            RuntimeError::InvalidMapWith(arity) => {
                write!(f, "attempt to map using a function of arity {}", arity)
            }
            RuntimeError::InvalidFilterWith(arity) => {
                write!(f, "attempt to filter using a function 0f arity {}", arity)
            }
            RuntimeError::TypeMissmatch { expected, got } => {
                write!(f, "expected type `{expected}`, got `{got}`")
            }
            RuntimeError::ExponentTooBig(n) => write!(f, "exponent too big: {}", n),
            RuntimeError::ZerothRoot => write!(f, "cannot take the 0th root"),
            RuntimeError::DivideByZero => write!(f, "cannot divide by zero"),
            RuntimeError::NoInverse => write!(f, "function is not inversible"),
            RuntimeError::InverseOfNonFunction => write!(f, "cannot invert a non-function"),
        }
    }
}

impl RuntimeError {
    pub fn note(&self) -> String {
        match self {
            RuntimeError::FunctionNotFound(_) => format!("check the docs for a list of functions"),
            RuntimeError::ListTypeMissmatch { .. } => {
                format!("ensure the list has elements of the same type")
            }
            RuntimeError::ListElementSizeMissmatch { .. } => {
                format!("ensure the list has elements of the same size")
            }
            RuntimeError::InvalidPop { .. } => {
                format!("ensure you are using the correct function or add more values to the stack")
            }
            RuntimeError::InvalidFoldWith(_) => format!("can only fold using binary operations"),
            RuntimeError::InvalidMapWith(_) => format!("can only map using unary operations"),
            RuntimeError::InvalidFilterWith(_) => format!("can only filter using unary operations"),
            RuntimeError::TypeMissmatch { .. } => format!(
                "ensure the function you're using works for the type of values on the stack"
            ),
            RuntimeError::ExponentTooBig(_) => format!("max is {} (u32::MAX)", u32::MAX),
            RuntimeError::ZerothRoot => format!("try filtering the 0s on the stack"),
            RuntimeError::DivideByZero => format!(
                "try filtering the 0s on the stack\nuse ε to produce a small number instead of 0"
            ),
            RuntimeError::NoInverse => format!("rethink your logic"),
            RuntimeError::InverseOfNonFunction => format!("ensure inverse comes after a function"),
        }
    }
}
