/* ------------------------------------------------------------
    Machine Pseudo-Code
    Project.Github: "https://github.com/kerryeon/mp_rust"
---------------------------------------------------------------
    Author:
        Name: "kerryeon"
        Email: "besqer996@gnu.ac.kr"
        Github: "https://github.com/kerryeon"
    Generated:
        Date: "2019-01-27"
------------------------------------------------------------ */

use std::fs::File;
use std::io::prelude::*;

pub fn read_file(path: &'static str) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(buf)
}
