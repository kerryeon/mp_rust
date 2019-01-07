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

const OP_CALL           : OpToken = OpToken::op     (":"     , 0);
const OP_TOWARD         : OpToken = OpToken::op     ("->"    , 1);
const OP_MATH           : OpToken = OpToken::op     ("$"     , 2);
const OP_STRING         : OpToken = OpToken::op     ("%"     , 2);
const OP_RAW            : OpToken = OpToken::op     ("!"     , 2);
const OP_NULL           : OpToken = OpToken::op     ("?"     , 2);

const MACRO_AND         : OpToken = OpToken::op     ("AND"   , 5);
const MACRO_OR          : OpToken = OpToken::op     ("OR"    , 4);
const MACRO_NOT         : OpToken = OpToken::op     ("NOT"   , 6);
const MACRO_IS          : OpToken = OpToken::op     ("IS"    , 3);

const MACRO_DEF         : OpToken = OpToken::op     ("def"   , 0);

const BPO               : OpToken = OpToken::op     ("("     , 7);
const BPC               : OpToken = OpToken::op     (")"     , 7);
const BSO               : OpToken = OpToken::op     ("["     , 7);
const BSC               : OpToken = OpToken::op     ("]"     , 7);
const BBO               : OpToken = OpToken::op     ("{"     , 7);
const BBC               : OpToken = OpToken::op     ("}"     , 7);

const INDENT_JUMP       : OpToken = OpToken::jump   ("\n");
const INDENT_TAB        : OpToken = OpToken::indent ("\t");
const INDENT_SPACE      : OpToken = OpToken::indent (" " );
const INDENT_COMMENT    : OpToken = OpToken::comment("#" );

pub type OpOrder = isize;
const OP_ORDER_BOTTOM: OpOrder = 15;

pub struct OpToken<'op> {
    pub token: &'op str,
    pub order: OpOrder,
    pub is_op: bool,
    pub is_indent: bool,
    pub is_end: bool,
}

impl <'op> OpToken <'op> {
    const fn new(
        token: &'op str,
        order: OpOrder,
        is_op: bool,
        is_indent: bool,
        is_end: bool,
    ) -> OpToken<'op> {
        OpToken {
            token,
            order,
            is_op,
            is_indent,
            is_end,
        }
    }

    const fn op(
        token: &'op str,
        order: OpOrder,
    ) -> OpToken<'op> {
        OpToken::new(
            token,
            order,
            true,
            false,
            false,
        )
    }

    const fn indent(
        token: &'op str,
    ) -> OpToken<'op> {
        OpToken::new(
            token,
            OP_ORDER_BOTTOM,
            false,
            true,
            false,
        )
    }

    const fn jump(
        token: &'op str,
    ) -> OpToken<'op> {
        OpToken::new(
            token,
            OP_ORDER_BOTTOM,
            false,
            false,
            true,
        )
    }

    const fn comment(
        token: &'op str,
    ) -> OpToken<'op> {
        OpToken::new(
            token,
            OP_ORDER_BOTTOM,
            false,
            false,
            true,
        )
    }
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
