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

const OP_INPLACE        : OpToken = OpToken::op_prime           (":"    , 1);
const OP_CALL           : OpToken = OpToken::op_prime           ("!"    , 2);
const OP_TOWARD         : OpToken = OpToken::op_prime           ("->"   , 3);
const OP_STRING         : OpToken = OpToken::op_prime           ("%"    , 4);
const OP_ANY            : OpToken = OpToken::op                 ("?"    , 14);

const OP_ADD            : OpToken = OpToken::op                 ("+"    , 8);
const OP_SUB            : OpToken = OpToken::op                 ("-"    , 8);
const OP_MUL            : OpToken = OpToken::op                 ("*"    , 8);
const OP_DIV            : OpToken = OpToken::op                 ("/"    , 8);
const OP_IDIV           : OpToken = OpToken::op                 ("//"   , 8);

const BPO               : OpToken = OpToken::shell_open         ("("    , 1);
const BSO               : OpToken = OpToken::shell_open         ("["    , 2);
const BBO               : OpToken = OpToken::shell_open         ("{"    , 3);
const BPC               : OpToken = OpToken::shell_close        (")"    , 1);
const BSC               : OpToken = OpToken::shell_close        ("]"    , 2);
const BBC               : OpToken = OpToken::shell_close        ("}"    , 3);
const STRING            : OpToken = OpToken::shell_string       ("\""   , 4);

const INDENT_JUMP       : OpToken = OpToken::jump               ("\n"   );
const INDENT_TAB        : OpToken = OpToken::indent             ("\t"   , 4);
const INDENT_SPACE      : OpToken = OpToken::indent             (" "    , 1);
const INDENT_COMMENT    : OpToken = OpToken::comment            ("#"    );
const SEPARATOR         : OpToken = OpToken::separator          (","    );

const MAGIC_STRING      : OpToken = OpToken::magic_code         ("\\\"" , MAGIC_CODE_STRING);
const MAGIC_TAB         : OpToken = OpToken::magic_code         ("\\t"  , MAGIC_CODE_TAB);

pub type OpOrder = u8;
pub type NumIndent = u8;
pub type ShellMap = u8;
pub type MagicMap = u8;
const OP_ORDER_TOP: OpOrder = 0;
const OP_ORDER_BOTTOM: OpOrder = 15;
const NO_SHELL: ShellMap = 0;
const NO_INDENT: NumIndent = 0;

const MAGIC_CODE_NONE       : MagicMap = 0;
pub const MAGIC_CODE_STRING : MagicMap = 1;
pub const MAGIC_CODE_TAB    : MagicMap = 2;

const SHELL_REMOVABLE: ShellMap = 1;

pub struct OpConfig {
    pub order: OpOrder,
    pub indent: NumIndent,
    pub is_op: bool,
    pub is_op_prime: bool,
    pub is_separator: bool,
    pub is_end: bool,
    pub is_string: bool,
    pub is_comment: bool,
    pub is_shell_closed: bool,
    pub shell_open: ShellMap,
    pub shell_close: ShellMap,
    pub magic_code: MagicMap,
}

impl OpConfig {
    const fn new(
        order: OpOrder,
        indent: NumIndent,
        is_op: bool,
        is_op_prime: bool,
        is_separator: bool,
        is_end: bool,
        is_string: bool,
        is_comment: bool,
        is_shell_closed: bool,
        shell_open: ShellMap,
        shell_close: ShellMap,
        magic_code: MagicMap,
    ) -> OpConfig {
        OpConfig {
            order,
            indent,
            is_op,
            is_op_prime,
            is_separator,
            is_end,
            is_string,
            is_comment,
            is_shell_closed,
            shell_open,
            shell_close,
            magic_code,
        }
    }

    pub const fn dummy() -> OpConfig {
        OpConfig::new(
            OP_ORDER_BOTTOM,
            NO_INDENT,
            false,
            false,
            false,
            false,
            false,
            false,
            true,
            NO_SHELL,
            NO_SHELL,
            MAGIC_CODE_NONE,
        )
    }

    pub const fn clone(&self) -> OpConfig {
        OpConfig::new(
            self.order,
            self.indent,
            self.is_op,
            self.is_op_prime,
            self.is_separator,
            self.is_end,
            self.is_string,
            self.is_comment,
            self.is_shell_closed,
            self.shell_open,
            self.shell_close,
            self.magic_code,
        )
    }

    pub const fn is_indent(&self) -> bool {
        self.indent != NO_INDENT
    }
    pub const fn is_shell_open(&self) -> bool {
        self.shell_open != NO_SHELL
    }
    pub const fn is_shell_close(&self) -> bool {
        self.shell_close != NO_SHELL
    }
    pub const fn is_shell_removable(&self) -> bool {
        self.shell_close == SHELL_REMOVABLE
    }
}

pub struct OpToken {
    pub token: &'static str,
    pub config: OpConfig,
}

impl OpToken {
    const fn new(
        token: &'static str,
        order: OpOrder,
        indent: NumIndent,
        is_op: bool,
        is_op_prime: bool,
        is_separator: bool,
        is_end: bool,
        is_string: bool,
        is_comment: bool,
        is_shell_closed: bool,
        shell_open: ShellMap,
        shell_close: ShellMap,
        magic_code: MagicMap,
    ) -> OpToken {
        OpToken {
            token,
            config: OpConfig::new(
                order,
                indent,
                is_op,
                is_op_prime,
                is_separator,
                is_end,
                is_string,
                is_comment,
                is_shell_closed,
                shell_open,
                shell_close,
                magic_code,
            )
        }
    }

    const fn op(
        token: &'static str,
        order: OpOrder,
    ) -> OpToken {
        OpToken::new(
            token,
            order,
            NO_INDENT,
            true,
            false,
            false,
            false,
            false,
            false,
            true,
            NO_SHELL,
            NO_SHELL,
            MAGIC_CODE_NONE,
        )
    }

    const fn op_prime(
        token: &'static str,
        order: OpOrder,
    ) -> OpToken {
        OpToken::new(
            token,
            order,
            NO_INDENT,
            true,
            true,
            false,
            false,
            false,
            false,
            true,
            NO_SHELL,
            NO_SHELL,
            MAGIC_CODE_NONE,
        )
    }

    const fn separator(
        token: &'static str,
    ) -> OpToken {
        OpToken::new(
            token,
            OP_ORDER_TOP,
            NO_INDENT,
            true,
            true,
            true,
            false,
            false,
            false,
            true,
            NO_SHELL,
            NO_SHELL,
            MAGIC_CODE_NONE,
        )
    }

    const fn no_op(
        token: &'static str,
        indent: NumIndent,
        is_end: bool,
        is_comment: bool,
    ) -> OpToken {
        OpToken::new(
            token,
            OP_ORDER_BOTTOM,
            indent,
            false,
            false,
            false,
            is_end,
            false,
            is_comment,
            true,
            NO_SHELL,
            NO_SHELL,
            MAGIC_CODE_NONE,
        )
    }

    const fn indent(
        token: &'static str,
        indent: NumIndent,
    ) -> OpToken {
        OpToken::no_op(
            token,
            indent,
            false,
            false,
        )
    }

    const fn jump(
        token: &'static str,
    ) -> OpToken {
        OpToken::no_op(
            token,
            NO_INDENT,
            true,
            false,
        )
    }

    const fn comment(
        token: &'static str,
    ) -> OpToken {
        OpToken::no_op(
            token,
            NO_INDENT,
            false,
            true,
        )
    }

    const fn shell_open(
        token: &'static str,
        map: ShellMap,
    ) -> OpToken {
        OpToken::new(
            token,
            OP_ORDER_TOP,
            NO_INDENT,
            true,
            false,
            false,
            false,
            false,
            false,
            false,
            map,
            NO_SHELL,
            MAGIC_CODE_NONE,
        )
    }

    const fn shell_string(
        token: &'static str,
        map: ShellMap,
    ) -> OpToken {
        OpToken::new(
            token,
            OP_ORDER_BOTTOM,
            NO_INDENT,
            true,
            false,
            false,
            false,
            true,
            false,
            false,
            map,
            map,
            MAGIC_CODE_NONE,
        )
    }

    const fn shell_close(
        token: &'static str,
        map: ShellMap,
    ) -> OpToken {
        OpToken::new(
            token,
            OP_ORDER_BOTTOM,
            NO_INDENT,
            true,
            false,
            false,
            false,
            false,
            false,
            true,
            NO_SHELL,
            map,
            MAGIC_CODE_NONE,
        )
    }

    const fn magic_code(
        token: &'static str,
        magic_code: MagicMap,
    ) -> OpToken {
        OpToken::new(
            token,
            OP_ORDER_BOTTOM,
            NO_INDENT,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            NO_SHELL,
            NO_SHELL,
            magic_code,
        )
    }
}

pub const OP_ORDER: [OpToken; 24] = [
    OP_INPLACE      ,
    OP_CALL         ,
    OP_TOWARD       ,
    OP_STRING       ,
    OP_ANY          ,
    OP_ADD          ,
    OP_SUB          ,
    OP_MUL          ,
    OP_DIV          ,
    OP_IDIV         ,
    BPO             ,
    BSO             ,
    BBO             ,
    BPC             ,
    BSC             ,
    BBC             ,
    STRING          ,
    INDENT_JUMP     ,
    INDENT_TAB      ,
    INDENT_SPACE    ,
    INDENT_COMMENT  ,
    SEPARATOR       ,
    MAGIC_STRING    ,
    MAGIC_TAB       ,
];

pub const OP_TOKEN: [OpToken; 20] = [
    OP_INPLACE      ,
    OP_CALL         ,
    OP_TOWARD       ,
    OP_STRING       ,
    OP_ANY          ,
    OP_ADD          ,
    OP_MUL          ,
    OP_IDIV         ,
    BPO             ,
    BSO             ,
    BBO             ,
    BPC             ,
    BSC             ,
    BBC             ,
    STRING          ,
    INDENT_JUMP     ,
    INDENT_TAB      ,
    INDENT_SPACE    ,
    INDENT_COMMENT  ,
    SEPARATOR       ,
];
