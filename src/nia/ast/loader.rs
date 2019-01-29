/* ------------------------------------------------------------
    Universal Task-Distributed Language
    Project.Github: "https://github.com/kerryeon/nia_rust"
---------------------------------------------------------------
    Author:
        Name: "kerryeon"
        Email: "besqer996@gnu.ac.kr"
        Github: "https://github.com/kerryeon"
    Generated:
        Date: "2019-01-29"
------------------------------------------------------------ */

use super::Module;
use crate::nia::error::ErrorCode;
use crate::nia::io::read_file;
use crate::nia::lexer::generate;
use crate::nia::parser::new_parser;

impl Module {
    pub fn from_file(path: &'static str) -> Result<Module, ErrorCode> {
        let source = match read_file(path) {
            Ok(s) => s,
            Err(e) => {
                println!("err! {}", e);
                return Err(0x1234)
            }
        };

        let mut root = new_parser(path);
        for token in generate(source.as_str()) {
            root.attach(token);
        }
        //root.tree();

        let mut module = Module::new(path);
        let mut traversal = root.traversal();
        while traversal.has_next_line {
            module.begin_line(traversal.get_indents());
            for node in traversal.next_line() {
                module.add_expr(node)
            }
            module.end_line();
        }
        Ok(module)
    }
}
