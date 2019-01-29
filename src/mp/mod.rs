/* ------------------------------------------------------------
    Machine Pseudo-Code
    Project.Github: "https://github.com/kerryeon/mp_rust"
---------------------------------------------------------------
    Author:
        Name: "kerryeon"
        Email: "besqer996@gnu.ac.kr"
        Github: "https://github.com/kerryeon"
    Generated:
        Date: "2019-01-03"
------------------------------------------------------------ */

mod error;
mod io;

mod ast;
mod config;
mod lexer;
mod parser;
mod token;

pub fn compile(path: &'static str) -> Result<ast::Module, error::ErrorCode> {
    ast::Module::from_file(path)
}
