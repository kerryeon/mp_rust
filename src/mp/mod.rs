/* ------------------------------------------------------------
    Machine Pseudo-Code
    Project.Github: !(https://github.com/kerryeon/mp_rust)
---------------------------------------------------------------
    Author:
        Name: kerryeon
        Email: besqer996@gnu.ac.kr
        Github: !(https://github.com/kerryeon)
    Generated:
        Date: 2019-01-03
------------------------------------------------------------ */

mod config;
mod tokenizer;

pub fn compile(source: &str) {
    for token in tokenizer::tokenize(source) {
        println!("Token [{}]", token);
    }
}
