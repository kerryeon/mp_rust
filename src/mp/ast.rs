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

use crate::mp::config;

pub type NodeNum = usize;

pub const NIL: NodeNum = 0;
pub const OFFSET_ROW: usize = 1;
pub const OFFSET_COL: usize = 1;

pub struct Node {
    pub token: String,
    pub token_len: usize,
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
    pub fn new(token: String, row: usize, col: usize) -> Node {
        let token_len = token.len();
        Node {
            token,
            token_len,
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

    pub fn root() -> Node {
        Node::new(String::new(), 0, 0)
    }

    pub const fn is_root(&self) -> bool {
        self.current == self.root
    }
}
