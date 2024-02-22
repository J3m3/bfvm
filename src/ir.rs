use crate::lexer::*;
use crate::op::*;
use crate::*;

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
