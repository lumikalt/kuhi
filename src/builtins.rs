use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::{err::RuntimeError, value::Value};

pub type Stack = Vec<Value>;
pub type RuntimeResult = Result<Vec<Value>, RuntimeError>;
pub type Func = fn(Stack) -> RuntimeResult;

lazy_static! {
    pub static ref BUILTINS: HashMap<char, Builtin> = HashMap::from([
        ('.', Builtin::new(dup, pop, 1)),
        ('+', Builtin::new(add, sub, 2))
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
    todo!()
}

fn pop(stack: Stack) -> RuntimeResult {
    todo!()
}

fn add(stack: Stack) -> RuntimeResult {
    let mut stack = stack.clone();
    let ([x, y], mut stack) = __pop_n(stack.to_vec());

    stack.push(x + y);
    Ok(stack)
}

fn sub(stack: Stack) -> RuntimeResult {
    todo!()
}

fn __pop_n<const N: usize>(stack: Vec<Value>) -> ([Value; N], Stack) {
    let mut stack = stack.clone();
    let mut values = [(); N].map(|_| unsafe { stack.pop().unwrap_unchecked() });
    values.reverse();
    (values, stack)
}
