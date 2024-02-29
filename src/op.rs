#[derive(Debug, PartialEq)]
pub enum OpKind {
    Inc,
    Dec,
    Left,
    Right,
    Input,
    Output,
    Jeq0Forward,
    Jne0Backward,
}

pub type Operand = i32;

#[derive(Debug, PartialEq)]
pub struct Op {
    pub kind: OpKind,
    pub operand: Operand,
}
