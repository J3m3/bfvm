mod interpreter;
mod ir;
mod lexer;
mod op;

pub use interpreter::interpret;


const MEM_SIZE: usize = 2usize.pow(16);
type Memory = [i32; MEM_SIZE];

type BackPatchingStack = Vec<i32>;
