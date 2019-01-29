/* ------------------------------------------------------------
    Machine Pseudo-Code
    Project.Github: "https://github.com/kerryeon/mp_rust"
---------------------------------------------------------------
    Author:
        Name: "kerryeon"
        Email: "besqer996@gnu.ac.kr"
        Github: "https://github.com/kerryeon"
    Generated:
        Date: "2019-01-07"
------------------------------------------------------------ */

pub type Token = String;

pub fn new(token: &str) -> Token {
    token.to_string()
}
