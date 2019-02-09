/* ------------------------------------------------------------
    Universal Task-Distributed Language
    Project.Github: "https://github.com/kerryeon/nia_rust"
---------------------------------------------------------------
    Author:
        Name: "kerryeon"
        Email: "besqer996@gnu.ac.kr"
        Github: "https://github.com/kerryeon"
    Generated:
        Date: "2019-01-26"
------------------------------------------------------------ */

mod print;
mod traversal;

use super::config;
use super::error::syntax;
use super::token::Token;

pub type NodeNum = usize;

pub const NIL: NodeNum = 0;
pub const OFFSET_ROW: usize = 1;
pub const OFFSET_COL: usize = 1;

pub struct Node {
    pub token: String,
    pub left: NodeNum,
    pub right: NodeNum,
    pub current: NodeNum,
    pub parent: NodeNum,
    pub root: NodeNum,
    pub config: config::OpConfig,

    pub row: usize,
    pub col: usize,
}

impl Node {
    pub fn new(token: String, row: usize, col: usize) -> Self {
        Self {
            token,
            left: NIL,
            right: NIL,
            current: NIL,
            parent: NIL,
            root: NIL,
            config: config::OpConfig::dummy(),
            row,
            col,
        }
    }

    pub fn root() -> Self {
        Self::new(String::new(), 0, 0)
    }

    pub const fn is_root(&self) -> bool {
        self.current == self.root
    }
}

pub enum ParseInsert {
    Left,
    LeftSwap,
    Right,
    RightRoot,
    None,
    Inline,
    InlineForce,
    Remove,
    OpenBracket,
    CloseBracket,
}
pub type ParseQuery = (NodeNum, bool);

pub struct Parser<'path> {
    pub nodes: Vec<Node>,
    pub now: usize,

    pub row: usize,
    pub col: usize,
    pub path: &'path str,
    pub is_comment: bool,
}

impl Node {
    fn from(token: String, row: usize, col: usize) -> Self {
        let mut node = Self::new(token, row, col);
        for op in config::OP_ORDER.iter() {
            if node.token == op.token {
                node.config = op.config.clone();
                break
            }
        }
        node
    }

    fn insert_left(&mut self, node: &mut Self, node_index: NodeNum, path: &str) -> ParseQuery {
        if self.left != NIL {
            syntax::inappropriate_token(path, node)
        }
        self.left = node_index;
        node.update(node_index, self.current, self.root)
    }
    fn insert_left_swap(&mut self, node: &mut Self, node_index: NodeNum) {
        node.left = self.current;
        node.update(node_index, self.parent, self.root);
    }
    fn insert_left_swap_alert(&mut self, node_from: NodeNum, node_index: NodeNum) -> ParseQuery {
        if self.left == node_from {
            self.left = node_index;
        } else {
            self.right = node_index;
        }
        (node_index, true)
    }
    fn insert_right(&mut self, node: &mut Self, node_index: NodeNum, path: &str) -> ParseQuery {
        if self.right != NIL {
            syntax::inappropriate_token(path, node)
        }
        self.right = node_index;
        node.update(node_index, self.current, self.root)
    }
    fn open_bracket(&mut self, node: &mut Self, node_index: NodeNum, path: &str) -> ParseQuery {
        node.token.clear();
        self.insert_right(node, node_index, path)
    }
    fn insert_right_root(&mut self, node: &mut Self, node_index: NodeNum) -> ParseQuery {
        self.right = node_index;
        node.token = String::new();
        node.update(node_index, self.current, node_index)
    }

    fn insert_inline(&mut self, node: &mut Self) -> ParseQuery {
        if self.is_root() && node.config.is_indent() {
            self.config.indent += node.config.indent;
            (self.current, false)
        } else {
            self.insert_inline_force(node)
        }
    }
    fn insert_inline_force(&mut self, node: &mut Self) -> ParseQuery {
        match node.config.magic_code {
            config::MAGIC_CODE_STRING => node.token = String::from("\""),
            config::MAGIC_CODE_TAB => node.token = String::from("\t"),
            _ => {},
        }
        self.token = format!("{}{}", self.token, node.token).to_string();
        (self.current, false)
    }

    fn update(&mut self, current: NodeNum, parent: NodeNum, root: NodeNum) -> ParseQuery {
        self.current = current;
        self.parent = parent;
        self.root = root;
        (current, true)
    }

    fn close_bracket(&mut self) -> ParseQuery {
        self.config.is_shell_closed = true;
        self.config.order = config::OP_ORDER_BOTTOM;
        (self.current, true)
    }
}

impl<'path> Parser<'path> {
    fn new(path: &'path str) -> Self {
        let node = Node::root();
        let mut nodes = Vec::new();
        nodes.push(node);
        Self {
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
            ParseInsert::Left         => parent.insert_left(&mut node, node_index, self.path),
            ParseInsert::LeftSwap     => self.insert_left_swap(&mut node, node_index, parent_index),
            ParseInsert::Right        => parent.insert_right(&mut node, node_index, self.path),
            ParseInsert::RightRoot    => parent.insert_right_root(&mut node, node_index),
            ParseInsert::None         => (parent.current, false),
            ParseInsert::Inline       => parent.insert_inline(&mut node),
            ParseInsert::InlineForce  => parent.insert_inline_force(&mut node),
            ParseInsert::Remove       => self.remove(parent_index),
            ParseInsert::OpenBracket  => parent.open_bracket(&mut node, node_index, self.path),
            ParseInsert::CloseBracket => parent.close_bracket(),
        };

        self.now = node_index;
        if allow_join {
            self.nodes.push(node);
        }
    }

    fn insert_left_swap(&mut self, node: &mut Node, node_index: NodeNum, parent_index: NodeNum) -> ParseQuery {
        let parent = &mut self.nodes[parent_index];
        parent.insert_left_swap(node, node_index);
        let grandparent_index = parent.parent;
        let grandparent = &mut self.nodes[grandparent_index];
        grandparent.insert_left_swap_alert(parent_index, node_index)
    }

    fn remove(&mut self, node_index: NodeNum) -> ParseQuery {
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

    fn get_position(&mut self, node: &Node) -> (NodeNum, ParseInsert) {
        let mut parent = &self.nodes[self.now];
        // 1. If parent is String
        if parent.config.is_string && ! parent.config.is_shell_closed {
            if parent.config.is_string && node.config.is_shell_close() {
                return (parent.current, ParseInsert::CloseBracket)
            }
            return (parent.current, ParseInsert::InlineForce)
        }
        // 2. is \n
        if node.config.is_end {
            self.is_comment = false;
            return (parent.root, ParseInsert::RightRoot)
        }
        // 3. is comment
        if node.config.is_comment || self.is_comment {
            self.is_comment = true;
            return (parent.current, ParseInsert::None)
        }
        // 4. If parent is root
        if parent.is_root()
        {
            return (parent.current, if node.config.is_indent() { ParseInsert::Inline } else { ParseInsert::Left })
        }
        // 5. If child is indent
        if node.config.is_indent() {
            return (parent.current, if parent.config.is_op { ParseInsert::None } else { ParseInsert::Inline })
        }
        if !node.config.is_op {
            // 6. If Parent isn't OP & Child isn't OP
            if !parent.config.is_op { return (parent.current, ParseInsert::Inline) }
            // 7. If Parent is    OP & Child isn't OP
            return (parent.current, ParseInsert::Right)
        }
        // 8. If Child is Opening Bracket
        if node.config.is_shell_open() { return (parent.current, ParseInsert::OpenBracket) }
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
                return (parent.current, ParseInsert::Remove)
            }
            return (parent.current, ParseInsert::CloseBracket)
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
                    (parent.right, ParseInsert::LeftSwap)
                },
                false => (parent.current, ParseInsert::LeftSwap),
            }
        }
        // 11. If Child is OP
        let mut is_upper = false;
        loop {
            if parent.is_root() { return (parent.left, ParseInsert::LeftSwap) }
            if node.config.order >= parent.config.order { break }
            parent = &self.nodes[parent.parent];
            is_upper = true;
        }
        match is_upper {
            true => (parent.right, ParseInsert::LeftSwap),
            false => (parent.current, ParseInsert::Right),
        }
    }
}

pub fn new_parser(path: &str) -> Parser {
    Parser::new(path)
}
