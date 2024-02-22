use crate::ir::*;
use crate::op::*;
use crate::*;

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
