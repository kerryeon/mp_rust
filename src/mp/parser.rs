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

use crate::mp::config;
use crate::mp::token::Token;

type NodeNum = usize;

struct Node {
    token: String,
    token_len: usize,
    left: NodeNum,
    right: NodeNum,
    current: NodeNum,
    parent: NodeNum,
    root: NodeNum,

    order: config::OpOrder,
    is_op: bool,
    is_indent: bool,
    is_end: bool,
}

const NIL: NodeNum = 0;

impl Node {
    fn from(token: String) -> Node {
        let mut node = Node::new(token);
        for op in config::OP_ORDER.iter() {
            if node.token == op.token {
                node.order = op.order;
                node.is_op = op.is_op;
                node.is_indent = op.is_indent;
                node.is_end = op.is_end;
                break
            }
        }
        node
    }

    fn new(token: String) -> Node {
        let token_len = token.len();
        Node {
            token,
            token_len,
            left: NIL,
            right: NIL,
            current: NIL,
            parent: NIL,
            root: NIL,
            order: -1,
            is_op: false,
            is_indent: false,
            is_end: false,
        }
    }

    fn root() -> Node {
        Node::new(String::new())
    }

    fn is_root(&self) -> bool {
        self.current == self.root
    }

    fn insert_left(&mut self, node: &mut Node, node_index: NodeNum) -> NodeNum {
        self.left = node_index;
        node.update(node_index, self.current, self.root)
    }
    fn insert_left_swap(&mut self, node: &mut Node, node_index: NodeNum) -> NodeNum {
        node.left = self.current;
        node.update(node_index, self.parent, self.root)
    }
    fn insert_left_swap_alert(&mut self, node_from: NodeNum, node_index: NodeNum) -> NodeNum {
        if self.left == node_from {
            self.left = node_index;
        } else {
            self.right = node_index;
        }
        node_index
    }
    fn insert_right(&mut self, node: &mut Node, node_index: NodeNum) -> NodeNum {
        self.right = node_index;
        node.update(node_index, self.current, self.root)
    }
    fn insert_right_root(&mut self, node: &mut Node, node_index: NodeNum) -> NodeNum {
        self.right = node_index;
        node.token = String::new();
        node.update(node_index, self.current, node_index)
    }
    fn insert_inline(&mut self, node: &mut Node) -> NodeNum {
        if self.is_indent {
            self.is_indent = false;
            self.token = node.token.clone();
        } else {
            self.token = format!("{}{}", self.token, node.token).to_string();
            self.token_len = self.token.len();
        };
        NIL
    }

    fn update(&mut self, current: NodeNum, parent: NodeNum, root: NodeNum) -> NodeNum {
        self.current = current;
        self.parent = parent;
        self.root = root;
        current
    }
}

pub struct AST {
    nodes: Vec<Node>,
    now: usize,
}

enum ASTInsert {
    Left,
    LeftSwap,
    Right,
    RightRoot,
    None,
    Inline,
}

impl AST {
    fn new() -> AST {
        let node = Node::root();
        let mut nodes = Vec::new();
        nodes.push(node);
        AST {
            nodes,
            now: NIL,
        }
    }

    pub fn attach(&mut self, token: Token) {
        let node_index = self.nodes.len();
        let mut node = Node::from(token);

        let (parent_index, insert) = self.get_position(&node);
        let parent = &mut self.nodes[parent_index];
        let node_index = match insert {
            ASTInsert::Left         => parent.insert_left(&mut node, node_index),
            ASTInsert::LeftSwap     => self.insert_left_swap(&mut node, node_index, parent_index),
            ASTInsert::Right        => parent.insert_right(&mut node, node_index),
            ASTInsert::RightRoot    => parent.insert_right_root(&mut node, node_index),
            ASTInsert::None         => NIL,
            ASTInsert::Inline       => parent.insert_inline(&mut node),
        };

        if node_index != NIL {
            self.now = node_index;
            self.nodes.push(node);
        }
    }

    pub fn tree(&self) {
        let root = &self.nodes[NIL];
        self._tree(root, "Root", 0);
    }

    fn _tree(&self, node: &Node, tag: &str, depth: usize) {
        for _ in 0..depth { print!("\t"); }
        println!("[{}] {}", tag, node.token);
        if node.left != NIL { self._tree(&self.nodes[node.left], "Left", depth+1); }
        if node.right != NIL {
            let child = &self.nodes[node.right];
            if node.is_root() { self._tree(child, "Root", depth); }
            else { self._tree(child, "Right", depth+1); }
        }
    }

    fn insert_left_swap(&mut self, node: &mut Node, node_index: NodeNum, parent_index: NodeNum) -> NodeNum {
        let parent = &mut self.nodes[parent_index];
        parent.insert_left_swap(node, node_index);
        let grandparent_index = parent.parent;
        let grandparent = &mut self.nodes[grandparent_index];
        grandparent.insert_left_swap_alert(parent_index, node_index)
    }

    fn get_position(&self, node: &Node) -> (NodeNum, ASTInsert) {
        let parent = &self.nodes[self.now];
        // 1. is \n
        if node.is_end { return (parent.root, ASTInsert::RightRoot) }
        // 2. If parent is root
        if parent.is_root()
        {
            return (parent.current, if node.is_indent { ASTInsert::Inline } else {ASTInsert::Left })
        }
        // 3. If child is indent
        if node.is_indent {
            return (parent.current, if parent.is_op { ASTInsert::None } else { ASTInsert::Inline })
        }
        if !node.is_op {
            // 4. If Parent isn't OP & Child isn't OP
            if !parent.is_op { return (parent.current, ASTInsert::Inline) }
            // 5. If Parent is    OP & Child isn't OP
            return (parent.current, ASTInsert::Right)
        }
        // 6. If Child is Closing Bracket
        // TODO
        // 7. If Child is OP
        while node.order < parent.order {
            let parent = &self.nodes[parent.parent];
            if parent.is_root() { return (parent.current, ASTInsert::Left) }
        }
        (parent.current, if node.order == parent.order { ASTInsert::Left } else { ASTInsert::LeftSwap })
    }
}

pub fn new_ast() -> AST {
    AST::new()
}
