mod codegen;

use std::fmt;
use std::mem::size_of;

use crate::ir::*;
use crate::op::*;
use crate::{BackPatchingStack, Memory};
use memmap2::{Mmap, MmapMut};

#[derive(Debug)]
pub struct JitCompileError {
    message: String,
}

impl JitCompileError {
    fn with_ip(ip: usize, message: &str) -> Self {
        JitCompileError {
            message: format!("{message} [IP:{ip}]"),
        }
    }

    fn new(message: &str) -> Self {
        JitCompileError {
            message: format!("{message}"),
        }
    }
}

impl fmt::Display for JitCompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "JIT COMPILE ERROR: {}", self.message)
    }
}

impl From<std::io::Error> for JitCompileError {
    fn from(value: std::io::Error) -> Self {
        JitCompileError {
            message: format!("{value}"),
        }
    }
}

impl From<JitCompileError> for std::io::Error {
    fn from(value: JitCompileError) -> Self {
        value.into()
    }
}

const AARCH64_INST_SIZE: usize = 4;

pub fn jit_compile(input: &str, memory: &mut Memory) -> Result<Mmap, JitCompileError> {
    const SZ: usize = AARCH64_INST_SIZE;
    let ops = generate_ops(input);
    let mut raw_code = Vec::new();
    let mut backpatches = BackPatchingStack::new();

    /* A dedicated register for data pointer: x19, which is callee-saved */
    // movz x19, #operand[..16], lsl #0
    // movk x19, #operand[16..32], lsl #16
    // movk x19, #operand[32..48], lsl #32
    // movk x19, #operand[48..64], lsl #48
    raw_code.extend_from_slice(&codegen::mov_x19_u64operand(memory.as_mut_ptr() as u64));

    for (idx, op) in ops.into_iter().enumerate() {
        let Op { kind, operand } = op;
        match kind {
            OpKind::Inc => {
                // mov w8, #operand[..16]
                // movk w8, #operand[16..], lsl #16
                raw_code.extend_from_slice(&codegen::mov_x8_i32operand(operand)); // now operand is in x8

                // ldr w9, [x19]
                raw_code.extend_from_slice(&codegen::ldr_w9_addrx19());
                // manually add nop because of load-use data hazard
                // this can be resolved through hardware, but it's uncertain whether older CPUs exist that cannot handle this problem.
                raw_code.extend_from_slice(&codegen::nop());
                // add w9, w9, w8
                raw_code.extend_from_slice(&codegen::add_w9_w9_w8());
                // str w9, [x19]
                raw_code.extend_from_slice(&codegen::str_w9_addrx19());
            }
            OpKind::Dec => {
                // mov w8, #operand[..16]
                // movk w8, #operand[16..], lsl #16
                raw_code.extend_from_slice(&codegen::mov_x8_i32operand(operand)); // now operand is in x8

                // ldr w9, [x19]
                raw_code.extend_from_slice(&codegen::ldr_w9_addrx19());
                // manually add nop because of load-use data hazard
                // this can be resolved through hardware, but it's uncertain whether older CPUs exist that cannot handle this problem.
                raw_code.extend_from_slice(&codegen::nop());
                // sub w9, w9, w8
                raw_code.extend_from_slice(&codegen::sub_w9_w9_w8());
                // str w9, [x19]
                raw_code.extend_from_slice(&codegen::str_w9_addrx19());
            }
            OpKind::Left => {
                // mov x8, #operand[..16]
                // movk x8, #operand[16..], lsl #16
                // FIX: check memory boundary
                raw_code.extend_from_slice(&codegen::mov_x8_i32operand(
                    operand * size_of::<Operand>() as i32,
                )); // now operand is in x8

                // sub x19, x19, x8
                raw_code.extend_from_slice(&codegen::sub_x19_x19_x8());
            }
            OpKind::Right => {
                // mov x8, #operand[..16]
                // movk x8, #operand[16..], lsl #16
                // FIX: check memory boundary
                raw_code.extend_from_slice(&codegen::mov_x8_i32operand(
                    operand * size_of::<Operand>() as i32,
                )); // now operand is in x8

                // add x19, x19, x8
                raw_code.extend_from_slice(&codegen::add_x19_x19_x8());
            }
            OpKind::Input => {
                for _ in 0..operand {
                    raw_code.extend_from_slice(&codegen::syscall_read());
                }
            }
            OpKind::Output => {
                // TODO: use buffer for optimization
                for _ in 0..operand {
                    raw_code.extend_from_slice(&codegen::syscall_write());
                }
            }
            OpKind::Jeq0Forward => {
                backpatches.push(raw_code.len() as i32);

                let placeholder: codegen::CondNearBranch = [0; SZ * 3];
                raw_code.extend_from_slice(&placeholder);
            }
            OpKind::Jne0Backward => {
                let Some(matching_byte_addr) = backpatches.pop() else {
                    return Err(JitCompileError::with_ip(
                        idx,
                        &format!(
                            "invalid program: `[` and `]` should match (`]` exceeds) [IDX:{idx}]"
                        ),
                    ));
                };

                // TODO: extend jump boundary (currently +-2^20 bytes = +-2^18 instructions)
                let curr_byte_addr = raw_code.len() as i32;
                let addr = matching_byte_addr as usize;
                let matching_inst =
                    &mut raw_code[addr..addr + size_of::<codegen::CondNearBranch>()];

                let base_amount = curr_byte_addr - matching_byte_addr;

                // TODO: check jump boundary
                let jez_amount = (base_amount >> 2) + 1; // equivalent to `base_amount / SZ + 1`, but faster
                let jnz_amount = -((base_amount >> 2) + 1);
                matching_inst.copy_from_slice(&codegen::cbz_x9_addrx19_immd19(jez_amount));
                raw_code.extend_from_slice(&codegen::cbnz_x9_addrx19_immd19(jnz_amount));
            }
        }
    }

    if !backpatches.is_empty() {
        let length = backpatches.len();
        for backpatch in backpatches {
            eprintln!(
                "{}",
                JitCompileError::with_ip(
                    backpatch as usize,
                    "invalid program: `[` and `]` should match"
                )
            );
        }
        return Err(JitCompileError::new(&format!("({length} `[`s left)",)));
    }
    // ret
    raw_code.extend_from_slice(&codegen::ret());

    let mut mmap = MmapMut::map_anon(raw_code.len())?;
    mmap.copy_from_slice(&raw_code);
    let mmap = mmap.make_exec()?;

    Ok(mmap)
}
