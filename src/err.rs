use std::fmt::{self, Display, Formatter};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SyntaxError {
    InvalidSymbol(char),
    /// true if `(`, false if `)`
    UnmatchedParenthesis(bool),
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SyntaxError::InvalidSymbol(_) => write!(f, "invalid symbol"),
            SyntaxError::UnmatchedParenthesis(_) => write!(f, "unmatched parenthesis"),
        }
    }
}

impl SyntaxError {
    pub fn note(&self) -> String {
        match self {
            SyntaxError::InvalidSymbol(_) => "check the docs for a list of valid symbols".to_owned(),
            SyntaxError::UnmatchedParenthesis(open) => format!(
                "there is a missing {} parenthesis in the code",
                if *open { "opening" } else { "closing" }
            ),
        }
    }
}

#[derive(Clone, Debug)]
pub enum RuntimeError {
    InvalidPop { len: usize, arity: usize },

    InvalidFoldWith(usize),
    InvalidMapWith(usize),
    InvalidFilterWith(usize),
    TypeMissmatch { expected: String, got: String }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeError::InvalidPop { len, arity } => write!(
                f,
                "attempt to pop {} values from a stack of size {}",
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
        }
    }
}

impl RuntimeError {
    pub fn note(&self) -> String {
        match self {
            RuntimeError::InvalidPop { len: _, arity: _ } => {
                "make sure you are using the correct function or add more values to the stack"
            }
            RuntimeError::InvalidFoldWith(_) => "can only fold using binary operations",
            RuntimeError::InvalidMapWith(_) => "can only map using unary operations",
            RuntimeError::InvalidFilterWith(_) => "can only filter using unary operations",
            RuntimeError::TypeMissmatch { expected: _, got: _ } => {
                "ensure the function you're using works for the type of values on the stack"
            }
        }
        .to_string()
    }
}
