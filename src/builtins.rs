use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::{err::RuntimeError, value::Value};

pub type Stack = Vec<Value>;
pub type RuntimeResult = Result<Vec<Value>, RuntimeError>;
pub type Func = fn(Stack) -> RuntimeResult;

lazy_static! {
    pub static ref BUILTINS: HashMap<char, Builtin> = HashMap::from([
        ('.', Builtin::new(dup, |_| Err(RuntimeError::NoInverse), 1)),
        (',', Builtin::new(pop, |_| Err(RuntimeError::NoInverse), 1)),
        ('↔', Builtin::new(flip, flip, 2)),
        ('+', Builtin::new(add, sub, 2)),
        ('-', Builtin::new(sub, add, 2)),
        ('×', Builtin::new(mul, div, 2)),
        ('÷', Builtin::new(div, mul, 2)),
        ('ⁿ', Builtin::new(pow, root, 2)),
        ('√', Builtin::new(root, pow, 2)),
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

    stack.push(x);
    stack.push(y);
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

/// As the program is ran from right to left, the resulting array will be in reverse.
fn __pop_n<const N: usize>(stack: Vec<Value>) -> ([Value; N], Stack) {
    let mut stack = stack.clone();
    // Already checked previously that the size is correct, so invariants hold
    let values = [(); N].map(|_| unsafe { stack.pop().unwrap_unchecked() });
    (values, dbg!(stack))
}
