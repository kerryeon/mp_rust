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

pub struct Tokenizer <'source> {
    begin: usize,
    count: usize,
    has_op: bool,
    op: Option<&'source str>,
    source: &'source str,
}

impl <'source> Tokenizer <'source> {
    fn new(source: &str) -> Tokenizer {
        Tokenizer {
            begin: 0,
            count: 0,
            has_op: false,
            op: None,
            source,
        }
    }
}

impl <'source> Iterator for Tokenizer <'source> {
    type Item = &'source str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.has_op {
            self.has_op = false;
            return self.op;
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
                        self.op = Some(&self.source[self.count - op.token.len()..self.count]);
                        return Some(ret)
                    }
                    return Some(current)
                }
            }
        }
        None
    }
}

pub fn tokenize(source: &str) -> Tokenizer {
    Tokenizer::new(source)
}
