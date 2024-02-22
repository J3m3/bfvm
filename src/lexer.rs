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
