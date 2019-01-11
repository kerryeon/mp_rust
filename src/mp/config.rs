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

const OP_CALL           : OpToken = OpToken::op             (":"    , 1);
const OP_TOWARD         : OpToken = OpToken::op             ("->"   , 2);
const OP_STRING         : OpToken = OpToken::op             ("%"    , 3);
const OP_RAW            : OpToken = OpToken::op             ("!"    , 3);
const OP_NULL           : OpToken = OpToken::op             ("?"    , 8);
const OP_ANY_MUL        : OpToken = OpToken::op             ("*"    , 8);

const MACRO_AND         : OpToken = OpToken::op             ("AND"  , 6);
const MACRO_OR          : OpToken = OpToken::op             ("OR"   , 5);
const MACRO_NOT         : OpToken = OpToken::op             ("NOT"  , 7);
const MACRO_IS          : OpToken = OpToken::op             ("IS"   , 4);

const MACRO_DEF         : OpToken = OpToken::op             ("def"  , 1);

const OP_ADD            : OpToken = OpToken::op             ("+"    , 8);
const OP_SUB            : OpToken = OpToken::op             ("-"    , 8);
const OP_DIV            : OpToken = OpToken::op             ("/"    , 8);
const OP_IDIV           : OpToken = OpToken::op             ("//"   , 8);

const BPO               : OpToken = OpToken::shell_open     ("("    , 1);
const BSO               : OpToken = OpToken::shell_open     ("["    , 2);
const BBO               : OpToken = OpToken::shell_open     ("{"    , 3);
const BPC               : OpToken = OpToken::shell_close    (")"    , 1);
const BSC               : OpToken = OpToken::shell_close    ("]"    , 2);
const BBC               : OpToken = OpToken::shell_close    ("}"    , 3);

const INDENT_JUMP       : OpToken = OpToken::jump           ("\n"   );
const INDENT_TAB        : OpToken = OpToken::indent         ("\t"   );
const INDENT_SPACE      : OpToken = OpToken::indent         (" "    );
const INDENT_COMMENT    : OpToken = OpToken::comment        ("#"    );

pub type OpOrder = usize;
const OP_ORDER_TOP: OpOrder = 0;
const OP_ORDER_BOTTOM: OpOrder = 15;
const NO_SHELL: usize = 0;

pub struct OpConfig {
    pub order: OpOrder,
    pub is_op: bool,
    pub is_indent: bool,
    pub is_end: bool,
    pub is_comment: bool,
    pub shell_open: usize,
    pub shell_close: usize,
}

impl OpConfig {
    const fn new(
        order: OpOrder,
        is_op: bool,
        is_indent: bool,
        is_end: bool,
        is_comment: bool,
        shell_open: usize,
        shell_close: usize,
    ) -> OpConfig {
        OpConfig {
            order,
            is_op,
            is_indent,
            is_end,
            is_comment,
            shell_open,
            shell_close,
        }
    }

    pub const fn dummy() -> OpConfig {
        OpConfig::new(
            OP_ORDER_BOTTOM,
            false,
            false,
            false,
            false,
            NO_SHELL,
            NO_SHELL,
        )
    }

    pub const fn clone(&self) -> OpConfig {
        OpConfig::new(
            self.order,
            self.is_op,
            self.is_indent,
            self.is_end,
            self.is_comment,
            self.shell_open,
            self.shell_close,
        )
    }

    pub const fn is_shell_open(&self) -> bool {
        self.shell_open != NO_SHELL
    }
    pub const fn is_shell_close(&self) -> bool {
        self.shell_close != NO_SHELL
    }
}

pub struct OpToken<'op> {
    pub token: &'op str,
    pub config: OpConfig,
}

impl<'op> OpToken<'op> {
    const fn new(
        token: &'op str,
        order: OpOrder,
        is_op: bool,
        is_indent: bool,
        is_end: bool,
        is_comment: bool,
        shell_open: usize,
        shell_close: usize,
    ) -> OpToken<'op> {
        OpToken {
            token,
            config: OpConfig::new(
                order,
                is_op,
                is_indent,
                is_end,
                is_comment,
                shell_open,
                shell_close,
            )
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
            false,
            NO_SHELL,
            NO_SHELL,
        )
    }

    const fn no_op(
        token: &'op str,
        is_indent: bool,
        is_end: bool,
        is_comment: bool,
    ) -> OpToken<'op> {
        OpToken::new(
            token,
            OP_ORDER_BOTTOM,
            false,
            is_indent,
            is_end,
            is_comment,
            NO_SHELL,
            NO_SHELL,
        )
    }

    const fn indent(
        token: &'op str,
    ) -> OpToken<'op> {
        OpToken::no_op(
            token,
            true,
            false,
            false,
        )
    }

    const fn jump(
        token: &'op str,
    ) -> OpToken<'op> {
        OpToken::no_op(
            token,
            false,
            true,
            false,
        )
    }

    const fn comment(
        token: &'op str,
    ) -> OpToken<'op> {
        OpToken::no_op(
            token,
            false,
            false,
            true,
        )
    }

    const fn shell_open(
        token: &'op str,
        map: usize,
    ) -> OpToken<'op> {
        OpToken::new(
            token,
            OP_ORDER_TOP,
            true,
            false,
            false,
            false,
            map,
            NO_SHELL,
        )
    }

    const fn shell_close(
        token: &'op str,
        map: usize,
    ) -> OpToken<'op> {
        OpToken::new(
            token,
            OP_ORDER_BOTTOM,
            true,
            false,
            false,
            false,
            NO_SHELL,
            map,
        )
    }
}

pub const OP_ORDER: [OpToken; 25] = [
    OP_CALL         ,
    OP_TOWARD       ,
    OP_STRING       ,
    OP_RAW          ,
    OP_NULL         ,
    OP_ANY_MUL      ,
    MACRO_AND       ,
    MACRO_OR        ,
    MACRO_NOT       ,
    MACRO_IS        ,
    MACRO_DEF       ,
    OP_ADD          ,
    OP_SUB          ,
    OP_DIV          ,
    OP_IDIV         ,
    BPO             ,
    BSO             ,
    BBO             ,
    BPC             ,
    BSC             ,
    BBC             ,
    INDENT_JUMP     ,
    INDENT_TAB      ,
    INDENT_SPACE    ,
    INDENT_COMMENT  ,
];

pub const OP_TOKEN: [OpToken; 18] = [
    OP_CALL         ,
    OP_TOWARD       ,
    OP_STRING       ,
    OP_RAW          ,
    OP_NULL         ,
    OP_ANY_MUL      ,
    OP_ADD          ,
    OP_IDIV         ,
    BPO             ,
    BSO             ,
    BBO             ,
    BPC             ,
    BSC             ,
    BBC             ,
    INDENT_JUMP     ,
    INDENT_TAB      ,
    INDENT_SPACE    ,
    INDENT_COMMENT  ,
];
