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

mod mp;

fn main() {
    mp::compile("./src/main.mp",
"def Main: a -> b# comment
\t\t\tPrint: hello ( world: (x->((y))))
");
}
