use crate::op::*;
use std::str::Chars;

pub struct Lexer<'a> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_lex_single_operations() {
        let ops = vec!['+', '-', '<', '>', ',', '.', '[', ']'];
        let expected = vec![
            OpKind::Inc,
            OpKind::Dec,
            OpKind::Left,
            OpKind::Right,
            OpKind::Input,
            OpKind::Output,
            OpKind::Jeq0Forward,
            OpKind::Jne0Backward,
        ];

        for (op, exp) in ops.into_iter().zip(expected.into_iter()) {
            let input = op.to_string();
            let mut lexer = Lexer::new(&input);
            assert_eq!(lexer.next(), Some(exp));
            assert_eq!(lexer.next(), None);
        }
    }

    #[test]
    fn should_ignore_invalid_characters() {
        let mut lexer = Lexer::new("a*b&c");
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn should_lex_sequences_of_operations() {
        let mut lexer = Lexer::new("+-<>[],.");
        let expected = vec![
            OpKind::Inc,
            OpKind::Dec,
            OpKind::Left,
            OpKind::Right,
            OpKind::Jeq0Forward,
            OpKind::Jne0Backward,
            OpKind::Input,
            OpKind::Output,
        ];

        for e in expected {
            assert_eq!(lexer.next(), Some(e));
        }
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn should_lex_sequences_of_operations_with_non_bf() {
        let mut lexer = Lexer::new("HI+I'M-NOT< A >VALID[BF],OP.");
        let expected = vec![
            OpKind::Inc,
            OpKind::Dec,
            OpKind::Left,
            OpKind::Right,
            OpKind::Jeq0Forward,
            OpKind::Jne0Backward,
            OpKind::Input,
            OpKind::Output,
        ];

        for e in expected {
            assert_eq!(lexer.next(), Some(e));
        }
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn should_lex_empty_input() {
        let mut lexer = Lexer::new("");
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn should_ignore_input_with_only_invalid_characters() {
        let mut lexer = Lexer::new("abcde");
        assert_eq!(lexer.next(), None);
    }
}
