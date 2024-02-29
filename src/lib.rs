mod interpreter;
mod ir;
mod lexer;
mod op;

pub use interpreter::interpret;

const MEM_SIZE: usize = 2usize.pow(16);
pub type Memory = [op::Operand; MEM_SIZE];

type BackPatchingStack = Vec<op::Operand>;
