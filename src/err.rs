use std::fmt::{self, Display, Formatter};

use thiserror::Error;

/// Location of a token in the source code
///
/// `offset`: number of characters from the beginning of the file \
/// `line`: number of lines from the beginning of the file \
/// `column`: number of characters from the beginning of the line
#[derive(Debug, Clone)]
pub struct Loc {
    pub offset: usize,
    pub line: usize,
    pub column: usize,
}

#[derive(Error, Debug)]
pub enum SyntaxError {
    InvalidSymbol(char),
    UnmatchedParenthesis,
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SyntaxError::InvalidSymbol(_) => write!(f, "invalid symbol"),
            SyntaxError::UnmatchedParenthesis => write!(f, "unmatched parenthesis"),
        }
    }
}

impl SyntaxError {
    pub fn note(&self) -> String {
        match self {
            SyntaxError::InvalidSymbol(_) => "check the docs for the list of valid symbols",
            SyntaxError::UnmatchedParenthesis => "there is a missing parenthesis in the code",
        }
        .to_owned()
    }
}

pub enum RuntimeError {
    InvalidPop { len: usize, arity: usize },

    InvalidFoldWith(usize),
    InvalidMapWith(usize),
    InvalidFilterWith(usize),
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
        }
        .to_string()
    }
}
