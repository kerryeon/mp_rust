/* ------------------------------------------------------------
    Machine Pseudo-Code
    Project.Github: !(https://github.com/kerryeon/mp_rust)
---------------------------------------------------------------
    Author:
        Name: kerryeon
        Email: besqer996@gnu.ac.kr
        Github: !(https://github.com/kerryeon)
    Generated:
        Date: 2019-01-05
------------------------------------------------------------ */

use crate::mp::ast::*;
use crate::mp::config;
use crate::mp::token::Token;

use crate::mp::error::syntax;

impl Node {
    fn from(token: String, row: usize, col: usize) -> Node {
        let mut node = Node::new(token, row, col);
        for op in config::OP_ORDER.iter() {
            if node.token == op.token {
                node.config = op.config.clone();
                break
            }
        }
        node
    }

    fn insert_left(&mut self, node: &mut Node, node_index: NodeNum, path: &str) -> ASTQuery {
        if self.left != NIL {
            syntax::inappropriate_token(path, node)
        }
        self.left = node_index;
        node.update(node_index, self.current, self.root)
    }
    fn insert_left_swap(&mut self, node: &mut Node, node_index: NodeNum) {
        node.left = self.current;
        node.update(node_index, self.parent, self.root);
    }
    fn insert_left_swap_alert(&mut self, node_from: NodeNum, node_index: NodeNum) -> ASTQuery {
        if self.left == node_from {
            self.left = node_index;
        } else {
            self.right = node_index;
        }
        (node_index, true)
    }
    fn insert_right(&mut self, node: &mut Node, node_index: NodeNum, path: &str) -> ASTQuery {
        if self.right != NIL {
            syntax::inappropriate_token(path, node)
        }
        self.right = node_index;
        node.update(node_index, self.current, self.root)
    }
    fn open_bracket(&mut self, node: &mut Node, node_index: NodeNum, path: &str) -> ASTQuery {
        node.token.clear();
        self.insert_right(node, node_index, path)
    }
    fn insert_right_root(&mut self, node: &mut Node, node_index: NodeNum) -> ASTQuery {
        self.right = node_index;
        node.token = String::new();
        node.update(node_index, self.current, node_index)
    }

    fn insert_inline(&mut self, node: &mut Node) -> ASTQuery {
        if node.config.is_indent() {
            self.config.indent += node.config.indent;
            (self.current, false)
        } else {
            self.insert_inline_force(node)
        }
    }
    fn insert_inline_force(&mut self, node: &mut Node) -> ASTQuery {
        match node.config.magic_code {
            config::MAGIC_CODE_STRING => node.token = String::from("\""),
            config::MAGIC_CODE_TAB => node.token = String::from("\t"),
            _ => {},
        }
        self.token = format!("{}{}", self.token, node.token).to_string();
        self.token_len = self.token.len();
        (self.current, false)
    }

    fn update(&mut self, current: NodeNum, parent: NodeNum, root: NodeNum) -> ASTQuery {
        self.current = current;
        self.parent = parent;
        self.root = root;
        (current, true)
    }

    fn close_bracket(&mut self) -> ASTQuery {
        self.config.is_shell_closed = true;
        (self.current, true)
    }
}

impl<'path> AST<'path> {
    fn new(path: &'path str) -> AST<'path> {
        let node = Node::root();
        let mut nodes = Vec::new();
        nodes.push(node);
        AST {
            nodes,
            now: NIL,
            row: OFFSET_ROW,
            col: OFFSET_COL,
            path,
            is_comment: false,
        }
    }

    fn new_node(&mut self, token: Token) -> Node {
        let node = Node::from(token, self.row, self.col);
        if node.config.is_end { self.row += 1; self.col = OFFSET_COL; }
        else { self.col += node.token.len(); }
        node
    }

    pub fn attach(&mut self, token: Token) {
        let node_index = self.nodes.len();
        let mut node = self.new_node(token);

        let (parent_index, insert) = self.get_position(&node);
        let parent = &mut self.nodes[parent_index];
        let (node_index, allow_join) = match insert {
            ASTInsert::Left         => parent.insert_left(&mut node, node_index, self.path),
            ASTInsert::LeftSwap     => self.insert_left_swap(&mut node, node_index, parent_index),
            ASTInsert::Right        => parent.insert_right(&mut node, node_index, self.path),
            ASTInsert::RightRoot    => parent.insert_right_root(&mut node, node_index),
            ASTInsert::None         => (parent.current, false),
            ASTInsert::Inline       => parent.insert_inline(&mut node),
            ASTInsert::InlineForce  => parent.insert_inline_force(&mut node),
            ASTInsert::Remove       => self.remove(parent_index),
            ASTInsert::OpenBracket  => parent.open_bracket(&mut node, node_index, self.path),
            ASTInsert::CloseBracket => parent.close_bracket(),
        };

        self.now = node_index;
        if allow_join {
            self.nodes.push(node);
        }
    }

    fn insert_left_swap(&mut self, node: &mut Node, node_index: NodeNum, parent_index: NodeNum) -> ASTQuery {
        let parent = &mut self.nodes[parent_index];
        parent.insert_left_swap(node, node_index);
        let grandparent_index = parent.parent;
        let grandparent = &mut self.nodes[grandparent_index];
        grandparent.insert_left_swap_alert(parent_index, node_index)
    }

    fn remove(&mut self, node_index: NodeNum) -> ASTQuery {
        let node = &mut self.nodes[node_index];
        let child_left_index = node.left;
        let child_right_index = node.right;
        let parent_index = node.parent;

        if child_left_index != NIL {
            let child = &mut self.nodes[child_left_index];
            syntax::inappropriate_token(self.path, child)
        }
        if child_right_index != NIL {
            let child = &mut self.nodes[child_right_index];
            child.parent = parent_index;
        }
        let parent = &mut self.nodes[parent_index];
        parent.right = child_right_index;
        (parent_index, false)
    }

    fn get_position(&mut self, node: &Node) -> (NodeNum, ASTInsert) {
        let mut parent = &self.nodes[self.now];
        // 1. If parent is String
        if parent.config.is_string && ! parent.config.is_shell_closed {
            if parent.config.is_string && node.config.is_shell_close() {
                return (parent.current, ASTInsert::CloseBracket)
            }
            return (parent.current, ASTInsert::InlineForce)
        }
        // 2. is \n
        if node.config.is_end {
            self.is_comment = false;
            return (parent.root, ASTInsert::RightRoot)
        }
        // 3. is comment
        if node.config.is_comment || self.is_comment {
            self.is_comment = true;
            return (parent.current, ASTInsert::None)
        }
        // 4. If parent is root
        if parent.is_root()
        {
            return (parent.current, if node.config.is_indent() { ASTInsert::Inline } else { ASTInsert::Left })
        }
        // 5. If child is indent
        if node.config.is_indent() {
            return (parent.current, if parent.config.is_op { ASTInsert::None } else { ASTInsert::Inline })
        }
        if !node.config.is_op {
            // 6. If Parent isn't OP & Child isn't OP
            if !parent.config.is_op { return (parent.current, ASTInsert::Inline) }
            // 7. If Parent is    OP & Child isn't OP
            return (parent.current, ASTInsert::Right)
        }
        // 8. If Child is Opening Bracket
        if node.config.is_shell_open() { return (parent.current, ASTInsert::OpenBracket) }
        // 9. If Child is Closing Bracket
        if node.config.is_shell_close() {
            while node.config.shell_close != parent.config.shell_open {
                parent = &self.nodes[parent.parent];
                if parent.config.is_shell_closed {
                    continue
                }
                if parent.is_root() {
                    syntax::opening_bracket_not_found(self.path, node)
                }
            }
            if node.config.is_shell_removable() {
                return (parent.current, ASTInsert::Remove)
            }
            return (parent.current, ASTInsert::CloseBracket)
        }
        // 10. If Child is Separator
        if node.config.is_separator {
            loop {
                if parent.is_root() {
                    syntax::inappropriate_token(self.path, node)
                }
                if parent.config.is_op { break }
                parent = &self.nodes[parent.parent];
            }
            return match parent.config.is_op_prime {
                true => {
                    if parent.right == NIL {
                        syntax::inappropriate_token(self.path, node)
                    }
                    (parent.right, ASTInsert::LeftSwap)
                },
                false => (parent.current, ASTInsert::LeftSwap),
            }
        }
        // 11. If Child is OP
        let mut is_upper = false;
        loop {
            if parent.is_root() { return (parent.left, ASTInsert::LeftSwap) }
            if node.config.order >= parent.config.order { break }
            parent = &self.nodes[parent.parent];
            is_upper = true;
        }
        match is_upper {
            true => (parent.right, ASTInsert::LeftSwap),
            false => (parent.current, ASTInsert::Right),
        }
    }
}

pub fn new_ast(path: &str) -> AST {
    AST::new(path)
}
