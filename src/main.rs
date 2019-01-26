/* ------------------------------------------------------------
    Machine Pseudo-Code
    Project.Github: !(https://github.com/kerryeon/mp_rust)
---------------------------------------------------------------
    Author:
        Name: kerryeon
        Email: besqer996@gnu.ac.kr
        Github: !(https://github.com/kerryeon)
    Generated:
        Date: 2019-01-03
------------------------------------------------------------ */

//mod console;
mod mp;

fn main() {
    //let cmd = console::Console::new();
    //cmd.begin_interactive();
    //cmd.compile("./src/main.mp",
    /*mp::compile("./src/main.mp",
"Main: a -> b# comment
\t\t\tPrint: hello ( world: (x->((y))))
");
    mp::compile("/src/math.mp",
    "c: a/b\n");
    mp::compile("/src/test1.mp",
    "a -> b\n");
    mp::compile("/src/test2.mp",
    "a -> b\n");
    mp::compile("/src/separator.mp",
    "a: b, c, d, e, -> f\n");
    mp::compile("/src/foo_call.mp",
    "Main:\n\tfoo: 3, 4\n");*/
    mp::compile("/src/foo_call.mp",
    "\tPrint! Format! x, y, z, w\n  ");
}
