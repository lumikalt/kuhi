use std::fmt::{self, Display, Formatter};

use rug::{Complex, Integer, Rational};

use crate::err::SyntaxError;

/// Location of a token in the source code
///
/// `start` : characters from the beginning of the file \
/// `end`   : characters from the beginning of the file \
/// `line`  : lines from the beginning of the file \
/// `column`: characters from the beginning of the line
#[derive(Debug, Clone)]
pub struct Loc {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug)]
pub enum Token {
    Integer(Integer),
    Rational(Rational),
    Complex(Complex),
    // String(String)
    // List(Vec<Token>),
    Function(Vec<(Token, Loc)>),

    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Factorial,
    Modulo,

    Spacing, // Otherwise Complex parsing consumes the previous token even when seperated by a space
    InvalidState,
}

impl Clone for Token {
    fn clone(&self) -> Self {
        use Token::*;
        match self {
            Integer(n) => Integer(n.clone()),
            Rational(r) => Rational(r.clone()),
            Complex(c) => Complex(c.clone()),

            Function(tokens) => Function(tokens.clone()),
            Add => Add,
            Subtract => Subtract,
            Multiply => Multiply,
            Divide => Divide,
            Power => Power,
            Factorial => Factorial,
            Modulo => Modulo,

            Spacing => Spacing,
            InvalidState => InvalidState,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use Token::*;

        match self {
            Integer(n) => write!(f, "{}", n),
            Rational(r) => write!(f, "{}", r),
            Complex(c) => write!(f, "{}", c),
            Function(tokens) => {
                write!(f, "(")?;
                for token in tokens {
                    write!(f, "{} ", token.0)?;
                }
                write!(f, ")")
            }

            Add => write!(f, "+"),
            Subtract => write!(f, "-"),
            Multiply => write!(f, "×"),
            Divide => write!(f, "÷"),
            Power => write!(f, "ⁿ"),
            Factorial => write!(f, "!"),
            Modulo => write!(f, "◿"),

            Spacing => write!(f, ""),
            InvalidState => write!(f, "<InvalidState>"),
        }
    }
}

pub fn parse(input: &str) -> Result<Vec<(Token, Loc)>, (SyntaxError, Loc, Vec<(Token, Loc)>)> {
    let mut tokens: Vec<(Token, Loc)> = Vec::new();
    let mut chars = input.chars().peekable();
    // Track current char
    let mut loc = Loc {
        start: 0,
        end: 0,
        line: 1,
        column: 1,
    };

    while let Some(c) = chars.next() {
        let token = match c {
            ' ' | '\r' => {
                loc.start += 1;
                loc.end += 1;
                loc.column += 1;

                Token::Spacing
            }
            '\n' => {
                loc.start += 1;
                loc.end += 1;
                loc.line += 1;
                loc.column = 1;

                Token::Spacing
            }

            '0'..='9' => (|| {
                let mut number = Integer::from(c.to_digit(10).unwrap());
                while let Some('0'..='9') = chars.peek() {
                    number *= 10;
                    number += Integer::from(chars.next().unwrap().to_digit(10).unwrap());

                    loc.end += 1;
                    loc.column += 1;
                }
                if let Some('.') = chars.peek() {
                    loc.end += 1;
                    loc.column += 1;

                    let mut denominator = Integer::from(1);
                    let mut decimal = Integer::from(0);
                    // peek two characters forward
                    if let Some('0'..='9') = chars.clone().nth(1) {
                        chars.next();
                        decimal *= 10;
                        decimal += Integer::from(chars.next().unwrap().to_digit(10).unwrap());
                        denominator *= 10;

                        loc.end += 1;
                        loc.column += 1;
                    } else {
                        // Undo changes
                        loc.end -= 1;
                        loc.column -= 1;

                        return Token::Integer(number);
                    }
                    while let Some('0'..='9') = chars.peek() {
                        decimal *= 10;
                        decimal += Integer::from(chars.next().unwrap().to_digit(10).unwrap());
                        denominator *= 10;

                        loc.end += 1;
                        loc.column += 1;
                    }
                    Token::Rational(Rational::from((
                        number * denominator.clone() + decimal,
                        denominator,
                    )))
                } else {
                    Token::Integer(number)
                }
            })(),
            'i' => (|| {
                // Consume previous token (if any) when it's a Real type
                // to get the real part
                let prev = tokens.clone();
                let otherwise = (Token::InvalidState.clone(), loc.clone());
                let prev = &prev.last().unwrap_or(&otherwise);

                let real = match &prev.0 {
                    Token::Integer(n) => Rational::from((n, 1)),
                    Token::Rational(r) => r.clone(),
                    Token::Spacing | Token::InvalidState | _ => Rational::from((0, 1)),
                };

                // Get imaginary part
                let mut number = Integer::from(1);
                if let Some('0'..='9') = chars.peek() {
                    number = Integer::from(chars.next().unwrap().to_digit(10).unwrap());

                    loc.end += 1;
                    loc.column += 1;
                }
                while let Some('0'..='9') = chars.peek() {
                    number *= 10;
                    number += Integer::from(chars.next().unwrap().to_digit(10).unwrap());

                    loc.end += 1;
                    loc.column += 1;
                }
                let imaginary = if let Some('.') = chars.peek() {
                    loc.end += 1;
                    loc.column += 1;

                    let mut denominator = Integer::from(1);
                    let mut decimal = Integer::from(0);
                    // peek two characters forward
                    if let Some('0'..='9') = chars.clone().nth(1) {
                        chars.next();
                        decimal *= 10;
                        decimal += Integer::from(chars.next().unwrap().to_digit(10).unwrap());
                        denominator *= 10;

                        loc.end += 1;
                        loc.column += 1;
                    } else {
                        // Undo changes
                        loc.end -= 1;
                        loc.column -= 1;

                        return Token::Integer(number);
                    }
                    while let Some('0'..='9') = chars.peek() {
                        decimal *= 10;
                        decimal += Integer::from(chars.next().unwrap().to_digit(10).unwrap());
                        denominator *= 10;

                        loc.end += 1;
                        loc.column += 1;
                    }
                    Rational::from((
                        number * denominator.clone() + decimal,
                        denominator,
                    ))
                } else {
                    Rational::from((number, 1))
                };

                Token::Complex(Complex::with_val(128, (real, imaginary)))
            })(),
            '+' => Token::Add,
            '-' => Token::Subtract,
            '×' => Token::Multiply,
            '÷' => Token::Divide,
            'ⁿ' => Token::Power,
            '◿' => Token::Modulo,

            '(' => {
                let mut depth = 1;
                let mut sub = String::new();
                while let Some(c) = chars.next() {
                    loc.end += 1;
                    loc.column += 1;

                    match c {
                        '(' => depth += 1,
                        ')' => depth -= 1,
                        '\n' => {
                            loc.line += 1;
                            loc.column = 1;
                            continue;
                        }
                        _ => {}
                    }
                    if depth == 0 {
                        break;
                    }
                    sub.push(c);
                }
                if depth != 0 {
                    loc.end = loc.start;
                    Err((
                        SyntaxError::UnmatchedParenthesis(false),
                        loc.clone(),
                        tokens.clone(),
                    ))?;
                }
                Token::Function(parse(sub.as_str())?)
            }
            ')' => Err((
                SyntaxError::UnmatchedParenthesis(false),
                loc.clone(),
                tokens.clone(),
            ))?,
            _ => Err((SyntaxError::InvalidSymbol(c), loc.clone(), tokens.clone()))?,
        };

        tokens.push((token, loc.clone()));

        loc.end += 1;
        loc.start = loc.end;
        loc.column += 1;
    }

    Ok(tokens)
}
