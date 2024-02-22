mod interpreter;
mod ir;
mod lexer;
mod op;

pub use interpreter::interpret;

use std::fmt;

#[derive(Debug)]
pub struct RuntimeError {
    message: String,
}

impl RuntimeError {
    fn new(ip: usize, message: &str) -> Self {
        RuntimeError {
            message: format!("{message} [IP:{ip}]"),
        }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RUNTIME ERROR: {}", self.message)
    }
}

const MEM_SIZE: usize = 2usize.pow(16);
type Memory = [i32; MEM_SIZE];

type BackPatchingStack = Vec<i32>;
