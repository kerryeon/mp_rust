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
    match mp::compile("a.mp") {
        Ok(_module) => {},
        Err(_e) => {},
    }
}
