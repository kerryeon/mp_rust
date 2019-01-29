/* ------------------------------------------------------------
    Universal Task-Distributed Language
    Project.Github: "https://github.com/kerryeon/nia_rust"
---------------------------------------------------------------
    Author:
        Name: "kerryeon"
        Email: "besqer996@gnu.ac.kr"
        Github: "https://github.com/kerryeon"
    Generated:
        Date: "2019-01-03"
------------------------------------------------------------ */

//mod console;
mod nia;

fn main() {
    match nia::compile("a.nia") {
        Ok(_module) => {},
        Err(_e) => {},
    }
}
