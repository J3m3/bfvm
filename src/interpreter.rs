use crate::ir::*;
use crate::op::*;
use crate::*;
use std::fmt;
use std::io::{Read, Write};

#[derive(Debug)]
pub struct RuntimeError {
    message: String,
}

impl RuntimeError {
    fn with_ip(ip: usize, message: &str) -> Self {
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

impl From<RuntimeError> for std::io::Error {
    fn from(value: RuntimeError) -> Self {
        value.into()
    }
}

pub fn interpret<R, W>(input: &str, mut stdin: R, mut stdout: W) -> Result<(), RuntimeError>
where
    R: Read,
    W: Write,
{
    let ops = generate_ops(input);
    let mut memory: Memory = [0; MEM_SIZE];
    let mut ip = 0; // TODO: use origianl ip without aggregation for better DX
    let mut dp = 0;

    while let Some(op) = ops.get(ip) {
        let operand = op.operand as usize;
        match op.kind {
            OpKind::Inc => memory[dp] += op.operand,
            OpKind::Dec => memory[dp] -= op.operand,
            OpKind::Left => {
                if dp < operand {
                    return Err(RuntimeError::with_ip(ip, "data pointer is negative"));
                }
                dp -= operand;
            }
            OpKind::Right => {
                if dp + operand > MEM_SIZE {
                    return Err(RuntimeError::with_ip(
                        ip,
                        "data pointer exceeded memory size",
                    ));
                }
                dp += operand;
            }
            OpKind::Input => {
                for _ in 0..operand {
                    let mut byte = [0; 1];
                    stdin.read_exact(&mut byte[0..1]).map_err(|e| {
                        RuntimeError::with_ip(ip, &format!("cannot read from stdin ({e})"))
                    })?;
                    memory[dp] = byte[0] as i32;
                }
            }
            OpKind::Output => {
                for _ in 0..operand {
                    let byte: u8 = memory[dp].try_into().map_err(|_| {
                        RuntimeError::with_ip(ip, "cannot reinterpret the byte into char")
                    })?;
                    if !byte.is_ascii() {
                        return Err(RuntimeError::with_ip(
                            ip,
                            "the value is not in the ASCII range",
                        ));
                    }
                    write!(stdout, "{}", char::from(byte)).map_err(|e| {
                        RuntimeError::with_ip(ip, &format!("cannot write to stdout ({e})"))
                    })?;
                }
            }
            OpKind::Jeq0Forward => {
                if MEM_SIZE < operand {
                    return Err(RuntimeError::with_ip(ip, "instruction pointer is negative"));
                }
                if memory[dp] == 0 {
                    ip = operand;
                    continue;
                }
            }
            OpKind::Jne0Backward => {
                if ops.len() < operand {
                    return Err(RuntimeError::with_ip(
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    // Helper function to simplify tests
    fn run_interpret(
        input_program: &str,
        input_data: &[u8],
    ) -> (Result<(), RuntimeError>, Vec<u8>) {
        let mut output_buffer = Vec::new();
        let result = interpret(input_program, Cursor::new(input_data), &mut output_buffer);
        (result, output_buffer)
    }

    #[test]
    fn should_interpret_basic_operations() {
        let (result, _output) = run_interpret("++>---<+", &[]);
        assert!(result.is_ok());
    }

    #[test]
    fn should_report_error_on_data_pointer_underflow() {
        let (result, _) = run_interpret("<", &[]);
        match result {
            Err(e) => assert!(e.message.contains("data pointer is negative")),
            _ => panic!("Expected a runtime error for data pointer underflow"),
        }
    }

    #[test]
    fn should_report_error_on_data_pointer_overflow() {
        let input = ">".repeat(MEM_SIZE + 1);
        let (result, _) = run_interpret(&input, &[]);
        match result {
            Err(e) => assert!(e.message.contains("data pointer exceeded memory size")),
            _ => panic!("Expected a runtime error for data pointer overflow"),
        }
    }

    #[test]
    fn should_interpret_loops_correctly() {
        let (result, _output) = run_interpret("++[->+<]", &[]);
        assert!(result.is_ok());
    }

    #[test]
    fn should_handle_input_and_output() {
        let input_program = ",.";
        let input_data = [65]; // ASCII code for 'A'
        let (result, output) = run_interpret(input_program, &input_data);
        assert!(result.is_ok());
        assert_eq!(output, input_data, "The output should match the input.");
    }

    #[test]
    fn should_interpret_complex_program() {
        let (result, _) = run_interpret("++[->+<]>++.", &[]);
        assert!(result.is_ok());
    }
}
