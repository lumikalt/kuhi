use std::collections::HashMap;

use lazy_static::lazy_static;
use rug::{Float, Integer};

use crate::{
    err::RuntimeError,
    value::{Value, PI},
};

pub type Stack = Vec<Value>;
pub type RuntimeResult = Result<Vec<Value>, RuntimeError>;
pub type Func = fn(Stack) -> RuntimeResult;

lazy_static! {
    pub static ref BUILTINS: HashMap<char, Builtin> = HashMap::from([
        ('.', Builtin::new(dup, |_| Err(RuntimeError::NoInverse), 1)),
        (',', Builtin::new(pop, |_| Err(RuntimeError::NoInverse), 1)),
        ('↕', Builtin::new(flip, flip, 2)),
        ('↺', Builtin::new(roll, roll, 3)),
        ('+', Builtin::new(add, sub, 2)),
        ('-', Builtin::new(sub, add, 2)),
        ('×', Builtin::new(mul, div, 2)),
        ('÷', Builtin::new(div, mul, 2)),
        ('ⁿ', Builtin::new(pow, root, 2)),
        ('√', Builtin::new(root, pow, 2)),
        ('◯', Builtin::new(sin, asin, 1)),
        ('ⓔ', Builtin::new(sinh, asinh, 1)),
        // ('Ⓞ', Builtin::new(sins, sins, 1)),
        ('ι', Builtin::new(iota, |_| Err(RuntimeError::NoInverse), 1)),
    ]);
}

/// # Builtin functions
///
/// `action` is what the function doeso n the stack
/// `inverse` is the reverse of what action would do
/// `arity` is how many stack elements are affected by a function
///
/// ## Examples
///
/// ```rust
///
/// ```
pub struct Builtin {
    pub action: Func,
    pub inverse: Func,
    pub arity: usize,
}

impl Builtin {
    pub fn new(action: Func, inverse: Func, arity: usize) -> Self {
        Self {
            action,
            inverse,
            arity,
        }
    }

    pub fn call(&self, stack: Stack) -> RuntimeResult {
        if self.arity > stack.len() {
            return Err(RuntimeError::InvalidPop {
                len: stack.len(),
                arity: self.arity,
            });
        }
        (self.action)(stack)
    }

    pub fn call_inverse(&self, stack: Stack) -> RuntimeResult {
        if self.arity > stack.len() {
            return Err(RuntimeError::InvalidPop {
                len: stack.len(),
                arity: self.arity,
            });
        }
        (self.inverse)(stack)
    }
}

fn dup(stack: Stack) -> RuntimeResult {
    let ([x], mut stack) = __pop_n(stack);

    stack.push(x.clone());
    stack.push(x);
    Ok(stack)
}

fn pop(stack: Stack) -> RuntimeResult {
    let ([_], stack) = __pop_n(stack);

    Ok(stack)
}

fn flip(stack: Stack) -> RuntimeResult {
    let ([y, x], mut stack) = __pop_n(stack);

    stack.push(y);
    stack.push(x);
    Ok(stack)
}

fn roll(stack: Stack) -> RuntimeResult {
    let ([z, y, x], mut stack) = __pop_n(stack);

    stack.push(x);
    stack.push(y);
    stack.push(z);
    Ok(stack)
}

fn add(stack: Stack) -> RuntimeResult {
    let ([y, x], mut stack) = __pop_n(stack);

    stack.push(x + y);
    Ok(stack)
}

fn sub(stack: Stack) -> RuntimeResult {
    let ([y, x], mut stack) = __pop_n(stack);

    stack.push(x + -y);
    Ok(stack)
}

fn mul(stack: Stack) -> RuntimeResult {
    let ([y, x], mut stack) = __pop_n(stack);

    stack.push(x * y);
    Ok(stack)
}

fn div(stack: Stack) -> RuntimeResult {
    let ([y, x], mut stack) = __pop_n(stack);

    stack.push(x * y.reciprocal());
    Ok(stack)
}

/// ⁿ2 3 => 9
fn pow(stack: Stack) -> RuntimeResult {
    let ([y, x], mut stack) = __pop_n(stack);

    stack.push(x.pow(&y));
    Ok(stack)
}

fn root(stack: Stack) -> RuntimeResult {
    let ([y, x], mut stack) = __pop_n(stack);

    stack.push(x.pow(&-y));
    Ok(stack)
}

fn sin(stack: Stack) -> RuntimeResult {
    let ([x], mut stack) = __pop_n(stack);

    match x {
        Value::Integer(x) => {
            stack.push(Value::Float(Float::with_val(128, x).sin()));
        }
        Value::Rational(x) => {
            stack.push(Value::Float(Float::with_val(128, x).sin()));
        }
        Value::Float(x) => {
            stack.push(Value::Float(x.sin()));
        }
        Value::Complex(x) => {
            stack.push(Value::Complex(x.sin()));
        }
        Value::Pi(r, sign) => match sign {
            1 => {
                stack.push(Value::Float(Float::with_val(128, r).sin_pi()));
            }
            -1 => {
                stack.push(Value::Float(
                    (Float::with_val(128, r) * PI.clone().recip()).sin(),
                ));
            }
            _ => unreachable!(),
        },

        Value::List(vals) => {
            let mut list = vec![];
            for val in vals.into_iter() {
                // TODO: Better error reporting
                list.push(sin(vec![val])?.first().unwrap().clone());
            }
            stack.push(Value::List(list));
        }
        _ => {
            return Err(RuntimeError::TypeMissmatch {
                expected: "Number".to_string(),
                got: x.types().join(", "),
            })
        }
    }

    Ok(stack)
}

fn asin(stack: Stack) -> RuntimeResult {
    let ([x], mut stack) = __pop_n(stack);

    match x {
        Value::Integer(x) => {
            stack.push(Value::Float(Float::with_val(128, x).asin()));
        }
        Value::Rational(x) => {
            stack.push(Value::Float(Float::with_val(128, x).asin()));
        }
        Value::Float(x) => {
            stack.push(Value::Float(x.asin()));
        }
        Value::Complex(x) => {
            stack.push(Value::Complex(x.asin()));
        }
        Value::Pi(r, sign) => match sign {
            1 => {
                stack.push(Value::Float(Float::with_val(128, r.clone() * PI.clone()).asin()));
            }
            -1 => {
                stack.push(Value::Float(
                    (Float::with_val(128, r) * PI.clone().recip()).asin(),
                ));
            }
            _ => unreachable!(),
        },
        _ => {
            return Err(RuntimeError::TypeMissmatch {
                expected: "Number".to_string(),
                got: x.types().join(", "),
            })
        }
    }

    Ok(stack)
}

fn sinh(stack: Stack) -> RuntimeResult {
    let ([x], mut stack) = __pop_n(stack);

    match x {
        Value::Integer(x) => {
            stack.push(Value::Float(Float::with_val(128, x).sinh()));
        }
        Value::Rational(x) => {
            stack.push(Value::Float(Float::with_val(128, x).sinh()));
        }
        Value::Float(x) => {
            stack.push(Value::Float(x.sinh()));
        }
        Value::Complex(x) => {
            stack.push(Value::Complex(x.sinh()));
        }
        _ => {
            return Err(RuntimeError::TypeMissmatch {
                expected: "Number".to_string(),
                got: x.types().join(", "),
            })
        }
    }

    Ok(stack)
}

fn asinh(stack: Stack) -> RuntimeResult {
    let ([x], mut stack) = __pop_n(stack);

    match x {
        Value::Integer(x) => {
            stack.push(Value::Float(Float::with_val(128, x).asinh()));
        }
        Value::Rational(x) => {
            stack.push(Value::Float(Float::with_val(128, x).asinh()));
        }
        Value::Float(x) => {
            stack.push(Value::Float(x.asinh()));
        }
        Value::Complex(x) => {
            stack.push(Value::Complex(x.asinh()));
        }
        _ => {
            return Err(RuntimeError::TypeMissmatch {
                expected: "Number".to_string(),
                got: x.types().join(", "),
            })
        }
    }

    Ok(stack)
}

fn iota(stack: Stack) -> RuntimeResult {
    let ([x], mut stack) = __pop_n(stack);
    let up_to = if let Value::Integer(x) = x {
        if !x.is_positive() {
            return Err(RuntimeError::InvalidIotaValue);
        }
        if let Some(x) = x.to_u64() {
            x
        } else {
            return Err(RuntimeError::InvalidIotaValue);
        }
    } else {
        return Err(RuntimeError::TypeMissmatch {
            expected: "Integer".to_string(),
            got: x.types().join(", "),
        });
    };

    stack.push(Value::List(
        (1..=up_to)
            .map(|n| Value::Integer(Integer::from(n)))
            .collect(),
    ));
    Ok(stack)
}

/// As the program is ran from right to left, the resulting array will be in reverse.
fn __pop_n<const N: usize>(stack: Vec<Value>) -> ([Value; N], Stack) {
    let mut stack = stack.clone();
    // Already checked previously that the size is correct, so invariants hold
    let values = [(); N].map(|_| unsafe { stack.pop().unwrap_unchecked() });
    (values, stack)
}
