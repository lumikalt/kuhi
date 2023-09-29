use std::fmt::{self, Display, Formatter};

use rug::{Integer, Rational};

use crate::err::{Loc, SyntaxError};

#[derive(Debug)]
pub enum Token {
    Number(Integer),
    Rational(Rational),
    Function(Vec<(Token, Loc)>),
    Plus,
    Minus,
}

impl Clone for Token {
    fn clone(&self) -> Self {
        match self {
            Token::Number(n) => Token::Number(n.clone()),
            Token::Rational(r) => Token::Rational(r.clone()),
            Token::Function(tokens) => Token::Function(tokens.clone()),
            Token::Plus => Token::Plus,
            Token::Minus => Token::Minus,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Token::Number(n) => write!(f, "{}", n),
            Token::Rational(r) => write!(f, "{}", r),
            Token::Function(tokens) => {
                write!(f, "(")?;
                for token in tokens {
                    write!(f, "{} ", token.0)?;
                }
                write!(f, ")")
            }
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
        }
    }
}

pub fn parse(input: &str) -> Result<Vec<(Token, Loc)>, (SyntaxError, Loc)> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    // Track current char
    let mut loc = Loc {
        offset: 0,
        line: 0,
        column: 0,
    };

    while let Some(c) = chars.next() {
        loc.offset += 1;
        loc.column += 1;

        let token = match c {
            ' ' | '\t' | '\r' => continue,
            '\n' => {
                loc.line += 1;
                loc.column = 0;
                continue;
            }
            '0'..='9' => {
                let mut number = Integer::from(c.to_digit(10).unwrap());
                while let Some('0'..='9') = chars.peek() {
                    number *= 10;
                    number += Integer::from(chars.next().unwrap().to_digit(10).unwrap());

                    loc.offset += 1;
                    loc.column += 1;
                }
                if let Some('.') = chars.peek() {
                    loc.offset += 1;
                    loc.column += 1;

                    chars.next();
                    let mut denominator = Integer::from(1);
                    let mut decimal = Integer::from(0);
                    while let Some('0'..='9') = chars.peek() {
                        decimal *= 10;
                        decimal += Integer::from(chars.next().unwrap().to_digit(10).unwrap());
                        denominator *= 10;

                        loc.offset += 1;
                        loc.column += 1;
                    }
                    Token::Rational(Rational::from((
                        number * denominator.clone() + decimal,
                        denominator,
                    )))
                } else {
                    Token::Number(number)
                }
            }
            '+' => Token::Plus,
            '-' => Token::Minus,
            '(' => {
                let mut depth = 1;
                let mut sub = String::new();
                while let Some(c) = chars.next() {
                    loc.offset += 1;
                    loc.column += 1;

                    match c {
                        '(' => depth += 1,
                        ')' => depth -= 1,
                        _ => {}
                    }
                    if depth == 0 {
                        break;
                    }
                    sub.push(c);
                }
                Token::Function(parse(sub.as_str())?)
            }
            ')' => {
                loc.offset -= 1;
                loc.column -= 1;

                Err((SyntaxError::UnmatchedParenthesis, loc.clone()))?
            }
            _ => {
                loc.offset -= 1;
                loc.column -= 1;

                Err((SyntaxError::InvalidSymbol(c), loc.clone()))?
            }
        };

        tokens.push((token, loc.clone()))
    }

    Ok(tokens)
}
