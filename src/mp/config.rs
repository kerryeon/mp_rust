/* ------------------------------------------------------------
    Machine Pseudo-Code
    Project.Github: !(https://github.com/kerryeon/mp_rust)
---------------------------------------------------------------
    Author:
        Name: kerryeon
        Email: besqer996@gnu.ac.kr
        Github: !(https://github.com/kerryeon)
    Generated:
        Date: 2019-01-04
------------------------------------------------------------ */

const OP_CALL           : OpToken = OpToken { token: ":"    , order: 0 , is_op: true , is_indent: false, is_end: false };
const OP_TOWARD         : OpToken = OpToken { token: "->"   , order: 1 , is_op: true , is_indent: false, is_end: false };
const OP_MATH           : OpToken = OpToken { token: "$"    , order: 2 , is_op: true , is_indent: false, is_end: false };
const OP_STRING         : OpToken = OpToken { token: "%"    , order: 2 , is_op: true , is_indent: false, is_end: false };
const OP_RAW            : OpToken = OpToken { token: "!"    , order: 2 , is_op: true , is_indent: false, is_end: false };
const OP_NULL           : OpToken = OpToken { token: "?"    , order: 2 , is_op: true , is_indent: false, is_end: false };

const MACRO_AND         : OpToken = OpToken { token: "AND"  , order: 5 , is_op: true , is_indent: false, is_end: false };
const MACRO_OR          : OpToken = OpToken { token: "OR"   , order: 4 , is_op: true , is_indent: false, is_end: false };
const MACRO_NOT         : OpToken = OpToken { token: "NOT"  , order: 6 , is_op: true , is_indent: false, is_end: false };
const MACRO_IS          : OpToken = OpToken { token: "IS"   , order: 3 , is_op: true , is_indent: false, is_end: false };

const MACRO_DEF         : OpToken = OpToken { token: "def"  , order: 0 , is_op: true , is_indent: false, is_end: false };

const BPO               : OpToken = OpToken { token: "("    , order: 7 , is_op: true , is_indent: false, is_end: false };
const BPC               : OpToken = OpToken { token: ")"    , order: 7 , is_op: true , is_indent: false, is_end: false };
const BSO               : OpToken = OpToken { token: "["    , order: 7 , is_op: true , is_indent: false, is_end: false };
const BSC               : OpToken = OpToken { token: "]"    , order: 7 , is_op: true , is_indent: false, is_end: false };
const BBO               : OpToken = OpToken { token: "{"    , order: 7 , is_op: true , is_indent: false, is_end: false };
const BBC               : OpToken = OpToken { token: "}"    , order: 7 , is_op: true , is_indent: false, is_end: false };

const INDENT_JUMP       : OpToken = OpToken { token: "\n"   , order: -1, is_op: false, is_indent: false, is_end: true  };
const INDENT_TAB        : OpToken = OpToken { token: "\t"   , order: -1, is_op: false, is_indent: true , is_end: false };
const INDENT_SPACE      : OpToken = OpToken { token: " "    , order: -1, is_op: false, is_indent: true , is_end: false };
const INDENT_COMMENT    : OpToken = OpToken { token: "#"    , order: -1, is_op: false, is_indent: false, is_end: false };

pub type OpOrder = isize;

pub struct OpToken<'op> {
    pub token: &'op str,
    pub order: OpOrder,
    pub is_op: bool,
    pub is_indent: bool,
    pub is_end: bool,
}

pub const OP_ORDER: [OpToken; 21] = [
    OP_CALL         ,
    OP_TOWARD       ,
    OP_MATH         ,
    OP_STRING       ,
    OP_RAW          ,
    OP_NULL         ,
    MACRO_AND       ,
    MACRO_OR        ,
    MACRO_NOT       ,
    MACRO_IS        ,
    MACRO_DEF       ,
    BPO             ,
    BPC             ,
    BSO             ,
    BSC             ,
    BBO             ,
    BBC             ,
    INDENT_JUMP     ,
    INDENT_TAB      ,
    INDENT_SPACE    ,
    INDENT_COMMENT  ,
];

pub const OP_TOKEN: [OpToken; 16] = [
    OP_CALL         ,
    OP_TOWARD       ,
    OP_MATH         ,
    OP_STRING       ,
    OP_RAW          ,
    OP_NULL         ,
    BPO             ,
    BPC             ,
    BSO             ,
    BSC             ,
    BBO             ,
    BBC             ,
    INDENT_JUMP     ,
    INDENT_TAB      ,
    INDENT_SPACE    ,
    INDENT_COMMENT  ,
];
