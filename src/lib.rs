use std::{fmt, str::Chars};

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

#[derive(Debug, PartialEq)]
pub struct Op {
    pub kind: OpKind,
    pub operand: i32,
}

const MEM_SIZE: usize = 2usize.pow(16);
type Memory = [i32; MEM_SIZE];

type BackPatchingStack = Vec<i32>;

struct Lexer<'a> {
    input_iter: Chars<'a>,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = OpKind;

    fn next(&mut self) -> Option<Self::Item> {
        self.input_iter.next().and_then(|c| match c {
            '+' => Some(OpKind::Inc),
            '-' => Some(OpKind::Dec),
            '<' => Some(OpKind::Left),
            '>' => Some(OpKind::Right),
            ',' => Some(OpKind::Input),
            '.' => Some(OpKind::Output),
            '[' => Some(OpKind::Jeq0Forward),
            ']' => Some(OpKind::Jne0Backward),
            _ => self.next(),
        })
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            input_iter: input.chars(),
        }
    }
}

pub fn generate_ops(input: &str) -> Vec<Op> {
    let lexer = Lexer::new(input);
    let mut addr_stack: BackPatchingStack = Vec::new();

    let commands = lexer
        .enumerate()
        .fold(Vec::new(), |mut acc: Vec<Op>, (idx, op_kind)| {
            if let Some(Op { kind, operand, .. }) = acc.last_mut() {
                if *kind == op_kind
                    && op_kind != OpKind::Jeq0Forward
                    && op_kind != OpKind::Jne0Backward
                {
                    *operand += 1;
                    return acc;
                }
            }

            if op_kind == OpKind::Jeq0Forward {
                addr_stack.push(acc.len() as i32);
            } else if op_kind == OpKind::Jne0Backward {
                let curr = acc.len() as i32;
                let matching = addr_stack.pop().expect(&format!(
                    "invalid program: `[` and `]` should match (`]` exceeds) [IDX:{idx}]"
                ));

                if let Some(op) = acc.get_mut(matching as usize) {
                    op.operand = curr + 1; // set the operand of `[`
                }
                acc.push(Op {
                    kind: op_kind,
                    operand: matching + 1, // set the operand of `]`
                });
                return acc;
            }

            acc.push(Op {
                kind: op_kind,
                operand: 1,
            });
            acc
        });

    if !addr_stack.is_empty() {
        panic!(
            "invalid program: `[` and `]` should match ({} `[`s left)",
            addr_stack.len()
        );
    }
    commands
}

pub fn interpret(input: &str) -> Result<(), RuntimeError> {
    use std::io::{stdin, stdout, Read, Write};

    let mut stdin = stdin().lock();
    let mut stdout = stdout().lock();

    let ops = generate_ops(input);
    let mut memory: Memory = [0; MEM_SIZE];
    let mut ip = 0;
    let mut dp = 0;

    while let Some(op) = ops.get(ip) {
        let operand = op.operand as usize;
        match op.kind {
            OpKind::Inc => memory[dp] += op.operand,
            OpKind::Dec => memory[dp] -= op.operand,
            OpKind::Left => {
                if dp < operand {
                    return Err(RuntimeError::new(ip, "data pointer is negative"));
                }
                dp -= operand;
            }
            OpKind::Right => {
                if dp > MEM_SIZE - operand {
                    return Err(RuntimeError::new(ip, "data pointer exceeded memory size"));
                }
                dp += operand;
            }
            OpKind::Input => {
                for _ in 0..operand {
                    let mut byte = [0; 1];
                    stdin.read_exact(&mut byte[0..1]).map_err(|e| {
                        RuntimeError::new(ip, &format!("cannot read from stdin ({e})"))
                    })?;
                    memory[dp] = byte[0] as i32;
                }
            }
            OpKind::Output => {
                for _ in 0..operand {
                    let byte: u8 = memory[dp].try_into().map_err(|_| {
                        RuntimeError::new(ip, "cannot reinterpret the byte into char")
                    })?;
                    if !byte.is_ascii() {
                        return Err(RuntimeError::new(ip, "the value is not in the ASCII range"));
                    }
                    write!(stdout, "{}", char::from(byte)).map_err(|e| {
                        RuntimeError::new(ip, &format!("cannot write to stdout ({e})"))
                    })?;
                }
            }
            OpKind::Jeq0Forward => {
                if MEM_SIZE < operand {
                    return Err(RuntimeError::new(ip, "instruction pointer is negative"));
                }
                if memory[dp] == 0 {
                    ip = operand;
                    continue;
                }
            }
            OpKind::Jne0Backward => {
                if ops.len() < operand {
                    return Err(RuntimeError::new(
                        ip,
                        "instruction pointer exceeded instruction buffer",
                    ));
                }
                if memory[dp] != 0 {
                    ip = operand;
                    continue;
                }
            }
        }
        ip += 1;
    }
    Ok(())
}
