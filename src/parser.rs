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
    Infinity,
    Epsilon,
    Pi(Rational),
    E(Rational, i32),

    Dup,
    Flip,
    Minus,

    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Root,
    Factorial,
    Modulo,

    Function(Vec<(Token, Loc)>),
    Inverse,

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

            Infinity => Infinity,
            Epsilon => Epsilon,
            Pi(r) => Pi(r.clone()),
            E(r, pow) => E(r.clone(), *pow),

            Dup => Dup,
            Flip => Flip,
            Minus => Minus,

            Add => Add,
            Subtract => Subtract,
            Multiply => Multiply,
            Divide => Divide,
            Power => Power,
            Root => Root,
            Factorial => Factorial,
            Modulo => Modulo,

            Function(tokens) => Function(tokens.clone()),
            Inverse => Inverse,

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

            Infinity => write!(f, "∞"),
            Epsilon => write!(f, "ε"),
            Pi(r) => {
                let (n, d) = r.clone().into_numer_denom();
                write!(
                    f,
                    "{}",
                    match (n.to_i8().unwrap(), d.to_i8().unwrap()) {
                        (1, 2) => todo!(),
                        (1, 1) => "π",
                        (2, 1) => "τ",
                        _ => unreachable!(),
                    }
                )
            }
            E(r, pow) => write!(f, "{}e^{}/{}", r.numer(), pow, r.denom()),

            Dup => write!(f, "."),
            Flip => write!(f, "↔"),
            Minus => write!(f, "⁻"),

            Add => write!(f, "+"),
            Subtract => write!(f, "-"),
            Multiply => write!(f, "×"),
            Divide => write!(f, "÷"),
            Power => write!(f, "ⁿ"),
            Root => write!(f, "√"),
            Factorial => write!(f, "!"),
            Modulo => write!(f, "◿"),

            Function(tokens) => {
                write!(f, "(")?;
                for token in tokens {
                    write!(f, "{} ", token.0)?;
                }
                write!(f, ")")
            }
            Inverse => write!(f, "⁻¹"),

            Spacing => write!(f, ""),
            InvalidState => write!(f, "<InvalidState>"),
        }
    }
}

pub fn parse(
    input: &str,
    loc: &mut Loc,
) -> Result<Vec<(Token, Loc)>, (SyntaxError, Loc, Vec<(Token, Loc)>)> {
    let mut tokens: Vec<(Token, Loc)> = Vec::new();
    let mut chars = input.chars().peekable();
    // Track token position for parsing errors.
    // On the run step, this is used for runtime error reporting, even when
    // the parse was successeful.

    while let Some(c) = chars.next() {
        // dbg!((c, loc.clone()));
        let token = match c {
            ' ' | '\r' => Token::Spacing,
            '\n' => {
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
                    Token::Integer(n) => {
                        // set start of loc to previous token's
                        loc.start = prev.1.start;
                        loc.column = prev.1.column;
                        tokens.pop();
                        Rational::from((n, 1))
                    }
                    Token::Rational(r) => {
                        // set start of loc to previous token's
                        loc.start = prev.1.start;
                        loc.column = prev.1.column;
                        tokens.pop();
                        r.clone()
                    }
                    Token::Spacing | _ => Rational::from((0, 1)),
                };

                // Get imaginary part
                let mut number = Integer::from(1);
                let mut sign = 1;
                if let Some('⁻') = chars.peek() {
                    sign = -1;
                    chars.next();
                    loc.end += 1;
                    loc.column += 1;
                }
                if let Some('.') = chars.peek() {
                    if sign == -1 {
                        return Err((
                            SyntaxError::InvalidSymbol('⁻'),
                            loc.clone(),
                            tokens.clone(),
                        ))?;
                    }
                    return Ok(Token::Complex(Complex::with_val(128, (real, 1))));
                }
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

                        while let Some('0'..='9') = chars.peek() {
                            decimal *= 10;
                            decimal += Integer::from(chars.next().unwrap().to_digit(10).unwrap());
                            denominator *= 10;

                            loc.end += 1;
                            loc.column += 1;
                        }
                    } else {
                        // Undo changes
                        loc.end -= 1;
                        loc.column -= 1;

                        decimal = number.clone();
                        denominator = Integer::from(1);
                    }
                    Rational::from((number * denominator.clone() + decimal, denominator))
                } else {
                    Rational::from((number, 1))
                };

                Ok(Token::Complex(Complex::with_val(
                    128,
                    (real, sign * imaginary),
                )))
            })()?,

            'π' => {
                // Consume previous token (if any) when it's a Real type
                let prev = tokens.clone();
                let otherwise = (Token::InvalidState.clone(), loc.clone());
                let prev = &prev.last().unwrap_or(&otherwise);

                let times = match &prev.0 {
                    Token::Integer(n) => {
                        // set start of loc to previous token's
                        loc.start = prev.1.start;
                        loc.column = prev.1.column;
                        tokens.pop();
                        Rational::from((n, 1))
                    }
                    Token::Rational(r) => {
                        // set start of loc to previous token's
                        loc.start = prev.1.start;
                        loc.column = prev.1.column;
                        tokens.pop();
                        r.clone()
                    }
                    Token::Spacing | Token::InvalidState | _ => Rational::from((0, 1)),
                };
                Token::Pi(times)
            }
            'τ' => {
                // Consume previous token (if any) when it's a Real type
                let prev = tokens.clone();
                let otherwise = (Token::InvalidState.clone(), loc.clone());
                let prev = &prev.last().unwrap_or(&otherwise);

                let times = match &prev.0 {
                    Token::Integer(n) => {
                        // set start of loc to previous token's
                        loc.start = prev.1.start;
                        loc.column = prev.1.column;
                        tokens.pop();
                        Rational::from((n, 1))
                    }
                    Token::Rational(r) => {
                        // set start of loc to previous token's
                        loc.start = prev.1.start;
                        loc.column = prev.1.column;
                        tokens.pop();
                        r.clone()
                    }
                    Token::Spacing | Token::InvalidState | _ => Rational::from((0, 1)),
                };
                Token::Pi(2 * times)
            }
            '∞' => Token::Infinity,
            'ε' => Token::Epsilon,

            '.' => Token::Dup,
            ',' => Err((SyntaxError::InvalidSymbol(','), loc.clone(), tokens.clone()))?,
            '↔' => Token::Flip,

            '+' => Token::Add,
            '-' => Token::Subtract,
            '×' => Token::Multiply,
            '÷' => Token::Divide,
            'ⁿ' => Token::Power,
            '√' => Token::Root,
            '!' => Token::Factorial,
            '◿' => Token::Modulo,

            '⁻' => {
                if let Some('¹') = chars.peek() {
                    chars.next();
                    loc.end += 1;
                    loc.column += 1;

                    Token::Inverse
                } else {
                    Token::Minus
                }
            }

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
                Token::Function(parse(sub.as_str(), loc)?)
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

    Ok(tokens
        .into_iter()
        .filter(|(token, _)| match token {
            Token::Spacing => false,
            _ => true,
        })
        .collect())
}
