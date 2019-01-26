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

pub struct ASTTraversal<'nodes> {
    nodes: &'nodes Vec<Node>,
    root: &'nodes Node,
    pub has_next_line: bool,
}

impl<'nodes> ASTTraversal<'nodes> {
    pub const fn get_indents(&self) -> u8 {
        self.root.config.indent
    }

    pub fn next_line(&mut self) -> ASTLine<'nodes> {
        let mut order: Vec<NodeNum> = Vec::new();
        let mut level: Vec<NodeNum> = Vec::new();
        let mut node = self.root;
        let mut first = false;

        self.has_next_line = self.root.right != NIL;
        self.root = &self.nodes[self.root.right];

        loop {
            if first {
                order.push(node.current);
            }

            if first && node.right != NIL {
                level.push(node.current);
                node = &self.nodes[node.right];
                continue
            }
            if node.left != NIL {
                node = &self.nodes[node.left];
                first = true;
                continue
            }

            node = match level.pop() {
                Some(n) => &self.nodes[n],
                None => break,
            };
            first = false;
        }

        ASTLine {
            nodes: self.nodes,
            order,
        }
    }

    fn new(nodes: &'nodes Vec<Node>) -> ASTTraversal<'nodes> {
        let root = &nodes[NIL];
        ASTTraversal {
            nodes,
            root,
            has_next_line: true,
        }
    }
}

pub struct ASTLine<'nodes> {
    nodes: &'nodes Vec<Node>,
    order: Vec<NodeNum>,
}

impl<'nodes> Iterator for ASTLine<'nodes> {
    type Item = &'nodes Node;

    fn next(&mut self) -> Option<Self::Item> {
        match self.order.pop() {
            Some(n) => Some(&self.nodes[n]),
            None => None,
        }
    }
}

impl<'path> AST<'path> {

    // Post-order
    pub fn traversal(&self) -> ASTTraversal {
        ASTTraversal::new(&self.nodes)
    }
}
