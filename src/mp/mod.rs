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

mod ast;
mod config;
mod error;
mod lexer;
mod parser;
mod token;

pub fn compile(filename: &str, source: &str) {
    let mut root = parser::new_ast(filename);
    for token in lexer::generate(source) {
        root.attach(token);
    }
    root.tree();

    let mut traversal = root.traversal();
    while traversal.has_next_line {
        println!("indents: {}", traversal.get_indents());
        for node in traversal.next_line() {
            println!("  token {}", node.token);
        }
    }
}
