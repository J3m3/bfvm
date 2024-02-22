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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_aggregate_basic_operations() {
        let input = "+++---";
        let expected = vec![
            Op {
                kind: OpKind::Inc,
                operand: 3,
            },
            Op {
                kind: OpKind::Dec,
                operand: 3,
            },
        ];
        let result = generate_ops(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn should_handle_loops() {
        let input = "[->+<]";
        let expected = vec![
            Op {
                kind: OpKind::Jeq0Forward,
                operand: 6,
            },
            Op {
                kind: OpKind::Dec,
                operand: 1,
            },
            Op {
                kind: OpKind::Right,
                operand: 1,
            },
            Op {
                kind: OpKind::Inc,
                operand: 1,
            },
            Op {
                kind: OpKind::Left,
                operand: 1,
            },
            Op {
                kind: OpKind::Jne0Backward,
                operand: 1,
            },
        ];
        let result = generate_ops(input);
        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic(expected = "invalid program: `[` and `]` should match (`]` exceeds)")]
    fn should_detect_unmatched_loops_excess_closing() {
        generate_ops("+++]");
    }

    #[test]
    #[should_panic(expected = "invalid program: `[` and `]` should match (1 `[`s left)")]
    fn should_detect_unmatched_loops_excess_opening() {
        generate_ops("[+++");
    }

    #[test]
    fn should_handle_complex_program() {
        let input = "++[->+<]>.-";
        let expected = vec![
            Op {
                kind: OpKind::Inc,
                operand: 2,
            },
            Op {
                kind: OpKind::Jeq0Forward,
                operand: 7,
            },
            Op {
                kind: OpKind::Dec,
                operand: 1,
            },
            Op {
                kind: OpKind::Right,
                operand: 1,
            },
            Op {
                kind: OpKind::Inc,
                operand: 1,
            },
            Op {
                kind: OpKind::Left,
                operand: 1,
            },
            Op {
                kind: OpKind::Jne0Backward,
                operand: 2,
            },
            Op {
                kind: OpKind::Right,
                operand: 1,
            },
            Op {
                kind: OpKind::Output,
                operand: 1,
            },
            Op {
                kind: OpKind::Dec,
                operand: 1,
            },
        ];
        let result = generate_ops(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn should_handle_empty_input() {
        let input = "";
        let expected: Vec<Op> = vec![];
        let result = generate_ops(input);
        assert_eq!(result, expected);
    }
}
