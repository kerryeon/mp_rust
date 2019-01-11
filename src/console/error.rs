/* ------------------------------------------------------------
    Machine Pseudo-Code
    Project.Github: !(https://github.com/kerryeon/mp_rust)
---------------------------------------------------------------
    Author:
        Name: kerryeon
        Email: besqer996@gnu.ac.kr
        Github: !(https://github.com/kerryeon)
    Generated:
        Date: 2019-01-11
------------------------------------------------------------ */

const SYMBOL: &str = "Console";

fn error(message: &str) {
    panic!("\n\
[{}Error] {}\n\
",
           SYMBOL, message);
}

pub fn io() {
    error("IO Error")
}
