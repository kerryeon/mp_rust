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

pub struct Tokenizer <'source> {
    begin: usize,
    count: usize,
    has_op: bool,
    op: &'source str,
    source: &'source str,
}

impl <'source> Tokenizer <'source> {
    fn new(source: &str) -> Tokenizer {
        Tokenizer {
            begin: 0,
            count: 0,
            has_op: false,
            op: "",
            source,
        }
    }
}

impl <'source> Iterator for Tokenizer <'source> {
    type Item = token::Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.has_op {
            self.has_op = false;
            return Some(token::new(self.op))
        }

        self.begin = self.count;
        while self.count < self.source.len() {
            self.count += 1;
            let current = &self.source[self.begin..self.count];
            for op in config::OP_TOKEN.iter() {
                if current.ends_with(op.token) {
                    let ret = &self.source[self.begin..self.count-op.token.len()];
                    if ret.len() > 0 {
                        self.has_op = true;
                        self.op = &self.source[self.count - op.token.len()..self.count];
                        return Some(token::new(ret))
                    }
                    return Some(token::new(current))
                }
            }
        }
        None
    }
}

pub fn tokenize(source: &str) -> Tokenizer {
    Tokenizer::new(source)
}
