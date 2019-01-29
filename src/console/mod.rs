/* ------------------------------------------------------------
    Universal Task-Distributed Language
    Project.Github: !(https://github.com/kerryeon/nia_rust)
---------------------------------------------------------------
    Author:
        Name: kerryeon
        Email: besqer996@gnu.ac.kr
        Github: !(https://github.com/kerryeon)
    Generated:
        Date: 2019-01-11
------------------------------------------------------------ */

use std::io;
use crate::nia;

mod error;

const CONSOLE_NAME: &str = "__main__";

pub struct Console {
}

impl Console {
    pub fn new() -> Console {
        Console {
        }
    }

    pub fn compile(&self, path: &str, source: &str) {
        nia::compile(path, source)
    }

    pub fn begin_interactive(&self) {
        let mut input: String = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_n) => self.compile(CONSOLE_NAME, &input.into_boxed_str()),
            Err(_e) => error::io(),
        }
    }
}
