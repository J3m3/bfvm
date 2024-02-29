mod interpreter;
mod ir;
mod jitc;
mod lexer;
mod op;

pub use interpreter::interpret;
pub use jitc::jit_compile;

pub const MEM_SIZE: usize = 2usize.pow(16);
pub type Memory = [op::Operand; MEM_SIZE];

type BackPatchingStack = Vec<op::Operand>;
