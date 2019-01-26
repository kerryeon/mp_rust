/* ------------------------------------------------------------
    Machine Pseudo-Code
    Project.Github: !(https://github.com/kerryeon/mp_rust)
---------------------------------------------------------------
    Author:
        Name: kerryeon
        Email: besqer996@gnu.ac.kr
        Github: !(https://github.com/kerryeon)
    Generated:
        Date: 2019-01-26
------------------------------------------------------------ */

use crate::mp::ast::*;

impl<'filename> AST<'filename> {
    pub fn tree(&self) {
        let root = &self.nodes[NIL];
        self._tree(root, "Root", 0);
    }

    fn _tree(&self, node: &Node, tag: &str, depth: usize) {
        for _ in 0..depth { print!("\t"); }
        if node.config.is_indent() {
            println!("[{}] {} indents", tag, node.config.indent)
        } else {
            println!("[{}] {}", tag, node.token)
        }
        if node.left != NIL { self._tree(&self.nodes[node.left], "Left", depth+1); }
        if node.right != NIL {
            let child = &self.nodes[node.right];
            if node.is_root() { self._tree(child, "Root", depth); }
            else { self._tree(child, "Right", depth+1); }
        }
    }
}
