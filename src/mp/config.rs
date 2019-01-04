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

const OP_CALL       : OpToken = OpToken { token: ":"    , order: 0 , is_op: true  };
const OP_TOWARD     : OpToken = OpToken { token: "->"   , order: 1 , is_op: true  };
const OP_MATH       : OpToken = OpToken { token: "$"    , order: 2 , is_op: true  };
const OP_STRING     : OpToken = OpToken { token: "%"    , order: 2 , is_op: true  };
const OP_RAW        : OpToken = OpToken { token: "!"    , order: 2 , is_op: true  };
const OP_NULL       : OpToken = OpToken { token: "?"    , order: 2 , is_op: true  };

const MACRO_AND     : OpToken = OpToken { token: "AND"  , order: 5 , is_op: true  };
const MACRO_OR      : OpToken = OpToken { token: "OR"   , order: 4 , is_op: true  };
const MACRO_NOT     : OpToken = OpToken { token: "NOT"  , order: 6 , is_op: true  };
const MACRO_IS      : OpToken = OpToken { token: "IS"   , order: 3 , is_op: true  };

const BPO           : OpToken = OpToken { token: "("    , order: 7 , is_op: true  };
const BPC           : OpToken = OpToken { token: ")"    , order: 7 , is_op: true  };
const BSO           : OpToken = OpToken { token: "["    , order: 7 , is_op: true  };
const BSC           : OpToken = OpToken { token: "]"    , order: 7 , is_op: true  };
const BBO           : OpToken = OpToken { token: "{"    , order: 7 , is_op: true  };
const BBC           : OpToken = OpToken { token: "}"    , order: 7 , is_op: true  };

const INDENT_JUMP   : OpToken = OpToken { token: "\n"   , order: -1, is_op: false };
const INDENT_TAB    : OpToken = OpToken { token: "\t"   , order: -1, is_op: false };
const INDENT_SPACE  : OpToken = OpToken { token: " "    , order: -1, is_op: false };

pub struct OpToken<'op> {
    pub token: &'op str,
    pub order: i32,
    pub is_op: bool,
}

pub const OP_ORDER: [OpToken; 16] = [
    OP_CALL     ,
    OP_TOWARD   ,
    OP_MATH     ,
    OP_STRING   ,
    OP_RAW      ,
    OP_NULL     ,
    MACRO_AND   ,
    MACRO_OR    ,
    MACRO_NOT   ,
    MACRO_IS    ,
    BPO         ,
    BPC         ,
    BSO         ,
    BSC         ,
    BBO         ,
    BBC         ,
];

pub const OP_TOKEN: [OpToken; 19] = [
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
    BPO             ,
    BPC             ,
    BSO             ,
    BSC             ,
    BBO             ,
    BBC             ,
    INDENT_JUMP     ,
    INDENT_TAB      ,
    INDENT_SPACE    ,
];
