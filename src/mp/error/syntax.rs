/* ------------------------------------------------------------
    Machine Pseudo-Code
    Project.Github: !(https://github.com/kerryeon/mp_rust)
---------------------------------------------------------------
    Author:
        Name: kerryeon
        Email: besqer996@gnu.ac.kr
        Github: !(https://github.com/kerryeon)
    Generated:
        Date: 2019-01-08
------------------------------------------------------------ */

use crate::mp::ast::Node;

const SYMBOL: &str = "Syntax";

fn error_target(message: &str, path: &str, target: &Node) {
    panic!("\n\
[{}Error] {}\n\
\t(about) {}\n\
\t(at) {}:{}:{}\n\
",
           SYMBOL, message, target.token, path, target.row, target.col);
}

pub fn opening_bracket_not_found(path: &str, target: &Node) {
    error_target("Opening bracket not found.", path, target)
}

pub fn inappropriate_token(path: &str, target: &Node) {
    let message = if target.config.is_op
    { "This operator can't be here." } else
    { "This operand can't be here." };
    error_target(message, path, target)
}
