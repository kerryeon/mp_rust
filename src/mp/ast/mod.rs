/* ------------------------------------------------------------
    Machine Pseudo-Code
    Project.Github: "https://github.com/kerryeon/mp_rust"
---------------------------------------------------------------
    Author:
        Name: "kerryeon"
        Email: "besqer996@gnu.ac.kr"
        Github: "https://github.com/kerryeon"
    Generated:
        Date: "2019-01-28"
------------------------------------------------------------ */

mod class;
mod class_type;
mod expression;
mod loader;
mod scope;

use self::class::{Class, ClassID};
use self::expression::Expression;
use self::scope::Line;

type Boolean = bool;
type Integer = i64;
type Float = f64;

pub struct Module {
    path: &'static str,
    attrs: Vec<ClassID>,
    classes: Vec<Class>,
    using: Vec<Module>,
    scopes: Vec<Line>,
    stack_expr: Vec<Expression>,
}
