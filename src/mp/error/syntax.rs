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

fn error_target(message: &str, filename: &str, target: &Node) {
    panic!("\n\
[{}Error] {}\n\
\t(about) {}\n\
\t(at) {}:{}:{}\n\
",
           SYMBOL, message, target.token, filename, target.row, target.col);
}

pub fn opening_bracket_not_found(filename: &str, target: &Node) {
    error_target("Opening bracket not found.", filename, target)
}

pub fn inappropriate_token(filename: &str, target: &Node) {
    let message = if target.config.is_op
    { "This operator can't be here." } else
    { "This operand can't be here." };
    error_target(message, filename, target)
}
