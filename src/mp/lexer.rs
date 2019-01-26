/* ------------------------------------------------------------
    Machine Pseudo-Code
    Project.Github: !(https://github.com/kerryeon/mp_rust)
---------------------------------------------------------------
    Author:
        Name: kerryeon
        Email: besqer996@gnu.ac.kr
        Github: !(https://github.com/kerryeon)
    Generated:
        Date: 2019-01-04
------------------------------------------------------------ */

use crate::mp::config;
use crate::mp::token;

use std::str::Chars;

pub struct Tokenizer<'source> {
    begin: usize,
    end: usize,
    count: usize,
    has_op: bool,
    was_indent: bool,
    return_first: bool,
    source: &'source str,
    chars: Chars<'source>,
}

impl<'source> Tokenizer<'source> {
    fn new(source: &str) -> Tokenizer {
        Tokenizer {
            begin: 0,
            end: 1,
            count: 0,
            has_op: false,
            was_indent: true,
            return_first: false,
            source,
            chars: source.chars(),
        }
    }

    fn is_numeric(c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_alphabet(c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')
    }

    fn is_special_char(c: char) -> bool {
        c == '.' || c == '_'
    }

    fn is_char(c: char) -> bool {
        Self::is_numeric(c) || Self::is_alphabet(c) || Self::is_special_char(c)
    }

    fn is_indent(c: char) -> bool {
        c == ' ' || c == '\t' || c == '\n'
    }

    fn is_useless(c: char) -> bool {
        c == '\r'
    }

    fn slice(&mut self, next_op: bool) -> token::Token {
        self.has_op = next_op;

        let ret = &self.source[self.begin..self.end];
        self.end = self.count;
        self.begin = self.end - 1;
        token::new(ret)
    }

    fn slice_first(&mut self) -> token::Token {
        self.return_first = false;
        let ret = &self.source[self.begin..self.end];

        self.begin += 1;
        self.end += 1;

        token::new(ret)
    }
}

impl<'source> Iterator for Tokenizer<'source> {
    type Item = token::Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.return_first {
            return Some(self.slice_first())
        }

        loop {
            let c = match self.chars.next() {
                None => break,
                Some(c) => c,
            };
            self.count += 1;

            if Self::is_useless(c) {
                self.begin += 1;
                self.end += 1;
                continue
            }

            if Self::is_indent(c) {
                if self.was_indent {
                    return Some(self.slice_first())
                }
                self.was_indent = true;
                self.return_first = true;
                return Some(self.slice(false))
            }

            if self.has_op {
                if Self::is_char(c) {
                    return Some(self.slice(false))
                } else {
                    let ret = &self.source[self.begin..self.end];
                    for op in config::OP_TOKEN.iter() {
                        if ret == op.token {
                            return Some(self.slice(true))
                        }
                    }
                }
            } else if !Self::is_char(c) {
                if self.was_indent {
                    self.was_indent = false;
                    self.has_op = true;
                    continue
                }
                return Some(self.slice(true))
            }

            self.was_indent = false;
            self.end = self.count;
        }
        None
    }
}

pub fn generate(source: &str) -> Tokenizer {
    Tokenizer::new(source)
}
