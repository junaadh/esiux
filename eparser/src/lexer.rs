use std::str::Chars;

use crate::{error::ParserErrorKind, ParseRes};

#[derive(Debug)]
pub struct Lexer<'a> {
    pub chars: Chars<'a>,
    pub len: usize,
    pub token_start: usize,
    pub line: usize,
}

impl<'a> Default for Lexer<'a> {
    fn default() -> Self {
        Self::new("")
    }
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let chars = source.chars();
        Self {
            token_start: 0,
            len: source.len(),
            chars,
            line: 1,
        }
    }

    pub fn advance(&mut self) -> Option<char> {
        self.chars.next().inspect(|x| {
            if x == &'\n' {
                self.line += 1;
            }
        }) /*map(|x| {
               if x == '\n' {
                   self.line += 1;
               }
               x
           })*/
    }

    pub fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    pub fn pos(&self) -> usize {
        self.len - self.chars.as_str().len()
    }

    pub fn advance_while<F>(&mut self, mut f: F)
    where
        F: FnMut(char) -> bool,
    {
        while !self.is_eof() && f(self.peek().unwrap()) {
            self.advance();
        }
    }

    pub fn advance_word(&mut self) {
        while !self.is_eof()
            && matches!(
                self.peek().unwrap(),
                'a'..='z' | 'A'..='Z' | '0'..='9' | '.' | '_'
            )
        {
            self.advance();
        }
    }

    pub fn advance_line(&mut self) {
        while !self.is_eof() && self.peek() != Some('\n') {
            self.advance();
        }
    }

    pub fn eat_char(&mut self, char: char) -> ParseRes<char> {
        match self.peek() {
            Some(x) if x == char => {
                self.advance();
                Ok(x)
            }
            Some(x) => Err(ParserErrorKind::Unexpected(
                Box::new(x),
                Box::new(char),
                self.line,
            )),
            None => Err(ParserErrorKind::Eof),
        }
    }

    pub fn reset_ptr(&mut self) {
        self.token_start = self.pos();
    }

    pub fn match_str(&mut self, str: &str) -> bool {
        let str_len = str.len();

        let chars = self.chars.as_str();
        if chars.len() < str_len {
            return false;
        }

        if str == &chars[..str_len] {
            for _ in str.chars() {
                self.advance();
            }
            true
        } else {
            false
        }
    }

    pub fn advance_untill(&mut self, word: &str) {
        for cur in word.chars() {
            while self.peek() != Some(cur) {
                self.advance();
            }
        }
        self.advance();
    }

    pub fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }
}
