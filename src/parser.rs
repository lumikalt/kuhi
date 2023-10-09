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

    List(Vec<Token>),
    _UnfinishedList(Vec<(Token, Loc)>),

    Scope(Vec<(Token, Loc)>),

    Dup,
    Pop,
    Flip,
    Minus,

    Function(Vec<(Token, Loc)>),
    FunctionCall(char),
    Inverse(Box<Token>, Loc),

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

            List(tokens) => List(tokens.clone()),
            _UnfinishedList(tokens) => _UnfinishedList(tokens.clone()),

            Scope(tokens) => Scope(tokens.clone()),

            Dup => Dup,
            Pop => Pop,
            Flip => Flip,
            Minus => Minus,

            Function(tokens) => Function(tokens.clone()),
            FunctionCall(f) => FunctionCall(f.clone()),
            Inverse(tok, loc) => Inverse(Box::new(*tok.clone()), loc.clone()),

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

            List(tokens) => write!(
                f,
                "[{}]",
                tokens
                    .into_iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
            _UnfinishedList(tokens) => {
                write!(f, "<")?;
                for token in tokens {
                    write!(f, "{} ", token.0)?;
                }
                write!(f, "…>")
            }

            Scope(tokens) => {
                write!(
                    f,
                    "{{{}}}",
                    tokens
                        .into_iter()
                        .map(|v| v.0.to_string())
                        .collect::<Vec<_>>()
                        .join(" ")
                )
            }

            Dup => write!(f, "."),
            Pop => write!(f, ","),
            Flip => write!(f, "↔"),
            Minus => write!(f, "⁻"),

            Function(tokens) => {
                write!(f, "(")?;
                for token in tokens {
                    write!(f, "{} ", token.0)?;
                }
                write!(f, ")")
            }
            FunctionCall(c) => write!(f, "{}", c),
            Inverse(tok, _) => write!(f, "⁻¹{}", tok.to_string()),

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
                // Consume previous token if it's a ⁻
                let prev = tokens.clone();
                let otherwise = (Token::InvalidState.clone(), loc.clone());
                let prev = &prev.last().unwrap_or(&otherwise);

                let sign = match &prev.0 {
                    Token::Minus => {
                        // set start of loc to previous token's
                        loc.start = prev.1.start;
                        loc.column = prev.1.column;
                        tokens.pop();
                        -1
                    }
                    Token::Spacing | Token::InvalidState | _ => 1,
                };

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
                        // Undo changes, end token
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
                    Token::Rational(
                        sign * Rational::from((
                            number * denominator.clone() + decimal,
                            denominator,
                        )),
                    )
                } else {
                    Token::Integer(sign * number)
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
                    Token::Spacing | Token::InvalidState | _ => Rational::from((1, 1)),
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
                        // tokens.pop();
                        Rational::from((n, 1))
                    }
                    Token::Rational(r) => {
                        // set start of loc to previous token's
                        loc.start = prev.1.start;
                        loc.column = prev.1.column;
                        tokens.pop();
                        r.clone()
                    }
                    Token::Spacing | Token::InvalidState | _ => Rational::from((1, 1)),
                };
                Token::Pi(2 * times)
            }
            '∞' => Token::Infinity,
            'ε' => Token::Epsilon,

            '‿' => {
                // Consume previous Value token
                let prev = tokens.clone();
                let otherwise = (Token::InvalidState.clone(), loc.clone());
                let prev = &prev.last().unwrap_or(&otherwise);

                let value = match &prev.0 {
                    Token::Integer(n) => {
                        // set start of loc to previous token's
                        loc.start = prev.1.start;
                        loc.column = prev.1.column;
                        tokens.pop();
                        Token::Integer(n.clone())
                    }
                    Token::Rational(r) => {
                        // set start of loc to previous token's
                        loc.start = prev.1.start;
                        loc.column = prev.1.column;
                        tokens.pop();
                        Token::Rational(r.clone())
                    }
                    Token::Complex(c) => {
                        // set start of loc to previous token's
                        loc.start = prev.1.start;
                        loc.column = prev.1.column;
                        tokens.pop();
                        Token::Complex(c.clone())
                    }
                    Token::Spacing | Token::InvalidState | _ => {
                        Err((SyntaxError::InvalidSymbol('‿'), loc.clone(), tokens.clone()))?
                    }
                };
                let value_loc = prev.1.clone();

                // Check second to last token for an unfinished list
                let prev = tokens.clone();
                let otherwise = (Token::InvalidState.clone(), loc.clone());
                let prev = &prev
                    .get(prev.len().overflowing_sub(2).0)
                    .unwrap_or(&otherwise);

                match &prev.0 {
                    Token::_UnfinishedList(tokens) => {
                        let mut tokens = tokens.clone();
                        // set start of loc to previous token's
                        loc.start = prev.1.start;
                        loc.column = prev.1.column;
                        tokens.pop();
                        Token::_UnfinishedList(tokens)
                    }
                    Token::Spacing | Token::InvalidState | _ => {
                        Token::_UnfinishedList(vec![(value, value_loc)])
                    }
                }
            }

            '.' => Token::Dup,
            ',' => Token::Pop,
            '↔' => Token::Flip,

            '⁻' => {
                if let Some('¹') = chars.peek() {
                    let c = chars.next();
                    loc.end += c.unwrap().len_utf8();
                    loc.column += 1;

                    // Temporary token value, post-processing will replace it
                    Token::Inverse(Box::new(Token::InvalidState), loc.clone())
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

            c => Token::FunctionCall(c),
        };

        tokens.push((token, loc.clone()));

        loc.end += c.len_utf8();
        loc.start = loc.end;
        loc.column += 1;
    }

    /* Post-processing */

    // Try to merge unfinished lists with the following value
    let mut tokens = tokens;
    let mut i = 0;
    while i < tokens.len() {
        if let Token::_UnfinishedList(mut unfinished) = tokens[i].0.clone() {
            let j = i + 1;
            while j < tokens.len() {
                // If there's spacing, parsing error
                if let Token::Spacing = tokens[j].0 {
                    Err((
                        SyntaxError::InvalidSymbol(' '),
                        tokens[j].1.clone(),
                        tokens.clone(),
                    ))?;
                }
                if let Token::_UnfinishedList(mut unfinished2) = tokens[j].0.clone() {
                    unfinished.append(&mut unfinished2);
                    tokens.remove(j);
                    continue;
                }
                unfinished.push(tokens[j].clone());
                tokens.remove(j);
                break;
            }
            tokens[i] = (
                Token::List(unfinished.into_iter().map(|v| v.0).collect()),
                tokens[i].1.clone(),
            );
        }
        i += 1;
    }

    tokens = tokens
        .into_iter()
        .filter(|(token, _)| match token {
            Token::Spacing => false,
            _ => true,
        })
        .collect();

    // Have inverse consume the next token
    let mut tokens = tokens;
    let mut i = 0;
    while i < tokens.len() {
        if let Token::Inverse(_, loc) = tokens[i].0.clone() {
            let j = i + 1;
            if j >= tokens.len() {
                Err((SyntaxError::LonelyInverse, loc.clone(), tokens.clone()))?;
            }
            tokens[i] = (
                Token::Inverse(Box::new(tokens[j].0.clone()), loc.clone()),
                loc,
            );
            tokens.remove(j);
        }
        i += 1;
    }

    Ok(tokens)
}
