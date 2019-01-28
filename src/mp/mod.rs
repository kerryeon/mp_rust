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

mod error;
mod io;

mod ast;
mod config;
mod lexer;
mod parser;
mod token;

pub fn compile(path: &'static str) {
    let source = match io::read_file(path) {
        Ok(s) => s,
        Err(e) => {
            println!("err! {}", e);
            return
        }
    };

    let mut root = parser::new_parser(path);
    for token in lexer::generate(source.as_str()) {
        root.attach(token);
    }
    //root.tree();

    let mut ast = ast::Module::new(path);
    let mut traversal = root.traversal();
    while traversal.has_next_line {
        ast.begin_line(traversal.get_indents());
        for node in traversal.next_line() {
            ast.add_expr(node)
        }
        ast.end_line();
    }
}
