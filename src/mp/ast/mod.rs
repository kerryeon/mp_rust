/* ------------------------------------------------------------
    Machine Pseudo-Code
    Project.Github: !(https://github.com/kerryeon/mp_rust)
---------------------------------------------------------------
    Author:
        Name: kerryeon
        Email: besqer996@gnu.ac.kr
        Github: !(https://github.com/kerryeon)
    Generated:
        Date: 2019-01-28
------------------------------------------------------------ */

mod class;
mod class_type;
mod expression;

use self::class::{Class, ClassID};
use self::class_type::TypeID;
use self::expression::Expression;
use crate::mp::config;
use crate::mp::parser::Node;

type Boolean = bool;
type Integer = i64;
type Float = f64;

struct Line {
    expr: Option<Expression>,
    indent: config::NumIndent,
}

pub struct Module {
    path: &'static str,
    attrs: Vec<Class>,
    using: Vec<Module>,
    scopes: Vec<Line>,
    stack_expr: Vec<Expression>,
}

impl Line {
    fn new(indent: config::NumIndent) -> Line {
        Line {
            expr: None,
            indent,
        }
    }
    fn from(cls: ClassID, indent: config::NumIndent) -> Line {
        Line {
            expr: Some(Expression::ClassExpr(cls)),
            indent,
        }
    }
}

impl Module {
    pub fn new(path: &'static str) -> Module {
        Module {
            path,
            attrs: vec!(),
            using: vec!(),
            scopes: vec!(),
            stack_expr: vec!(),
        }
    }
    pub fn begin_line(&mut self, num_indent: config::NumIndent) {
        println!("[new line] [{}]", num_indent);
        let line = Line::new(num_indent);
        self.add_attr(num_indent);
        self.scopes.push(line)
    }
    pub fn add_expr(&mut self, node: &Node) {
        let expr = self.convert_node_to_expr(node);
        self.stack_expr.push(expr)
    }
    pub fn end_line(&mut self) {
        assert!(self.stack_expr.len() <= 1);
        match self.stack_expr.pop() {
            Some(expr) => {
                match self.scopes.pop() {
                    Some(mut scope) => {
                        scope.expr = Some(expr);
                        self.scopes.push(scope)
                    },
                    None => panic!(),
                }
            },
            None => {},
        };
    }
    fn add_attr(&mut self, num_indent: config::NumIndent) {
        loop {
            let line = match self.scopes.pop() {
                Some(line) => line,
                None => match num_indent == config::NO_INDENT {
                    true => return,
                    false => panic!(),
                },
            };
            if line.indent < num_indent { self.scopes.push(line) }
            else if line.indent != config::NO_INDENT {
                let (cls, cls_indent) = match self.scopes.pop() {
                    Some(line) => match line.expr {
                        Some(expr) => match expr {
                            Expression::ClassExpr(cls) => (cls, line.indent),
                            _ => panic!(),
                        },
                        None => panic!(),
                    },
                    None => panic!(),
                };

                match line.expr {
                    Some(expr) => match expr {
                        Expression::ClassExpr(attr) => self.add_attr_in_class(cls, attr),
                        _ => panic!(),
                    },
                    None => {},
                }

                self.scopes.push(Line::from(cls, cls_indent));
                if line.indent > num_indent { continue }
            }
            break
        }
    }
    fn find_class(&mut self, name: String) -> ClassID {
        for (idx, attr) in self.attrs.iter().enumerate() {
            if name == attr.name {
                return idx
            }
        }
        let id: ClassID = self.attrs.len();
        let attr = Class::new(name, id);
        self.attrs.push(attr);
        id
    }
    fn add_attr_in_class(&mut self, cls: ClassID, attr: ClassID) {
        let cls = &mut self.attrs[cls];
        cls.attrs.push(attr)
    }

    fn get_class_type(&self, cls: ClassID) -> String {
        let cls = &self.attrs[cls];
        let t = &cls.type_id;
        match t {
            TypeID::Class(id) => (&self.attrs[id.clone()]).name.clone(),
            _ => t.as_str(),
        }
    }
}
