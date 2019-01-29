/* ------------------------------------------------------------
    Machine Pseudo-Code
    Project.Github: "https://github.com/kerryeon/mp_rust"
---------------------------------------------------------------
    Author:
        Name: "kerryeon"
        Email: "besqer996@gnu.ac.kr"
        Github: "https://github.com/kerryeon"
    Generated:
        Date: "2019-01-29"
------------------------------------------------------------ */

use super::class::{Class, ClassID};
use super::class_type::TypeID;
use super::Module;
use super::expression::Expression;
use crate::mp::config;
use crate::mp::parser::Node;

pub struct Line {
    expr: Option<Expression>,
    indent: config::NumIndent,
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
            classes: vec!(),
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
            else {
                let (cls, cls_indent) = match self.scopes.pop() {
                    Some(line) => match line.expr {
                        Some(expr) => match expr {
                            Expression::ClassExpr(cls) => (cls, line.indent),
                            _ => panic!(),
                        },
                        None => panic!(),
                    },
                    None => match line.indent == 0 {
                        true => {
                            match line.expr {
                                Some(expr) => match expr {
                                    Expression::ClassExpr(cls) => self.attrs.push(cls),
                                    _ => panic!(),
                                },
                                None => {},
                            }
                            break
                        },
                        false => panic!(),
                    },
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
    pub fn find_class(&mut self, name: String) -> ClassID {
        let mut scope = match self.get_class_scope() {
            Some(cls) => cls,
            None => return self.find_class_in_module(name),
        };
        loop {
            for attr in scope.attrs.iter().map(|id| &self.classes[*id]) {
                if attr.name == name {
                    return attr.current
                }
            }
            scope = match self.get_class_parent(scope) {
                Some(cls) => cls,
                None => return self.find_class_in_module(name),
            };
        }
    }
    pub fn get_class_type(&self, cls: ClassID) -> String {
        let cls = &self.classes[cls];
        let t = &cls.type_id;
        match t {
            TypeID::Class(id) => (&self.classes[*id]).name.clone(),
            _ => t.as_str(),
        }
    }

    fn add_attr_in_class(&mut self, cls: ClassID, attr: ClassID) {
        let cls = &mut self.classes[cls];
        cls.attrs.push(attr)
    }
    fn get_class_scope(&self) -> Option<&Class> {
        if self.scopes.len() >= 2 {
            match self.scopes.get(self.scopes.len() - 2) {
                Some(ref line) => match &line.expr {
                    Some(expr) => match expr {
                        Expression::ClassExpr(id) => Some(&self.classes[*id]),
                        _ => None,
                    }
                    None => None,
                },
                None => None,
            }
        } else { None }
    }
    fn get_class_parent(&self, cls: &Class) -> Option<&Class> {
        match cls.parent {
            Some(id) => Some(&self.classes[id]),
            None => None,
        }
    }
    fn find_class_in_module(&mut self, name: String) -> ClassID {
        for attr in (&self.attrs).iter().map(|id| &self.classes[*id]) {
            if attr.name == name {
                return attr.current
            }
        }
        self.new_class(name)
    }
    fn new_class(&mut self, name: String) -> ClassID {
        println!("    [new class] {}", name);
        let id: ClassID = self.classes.len();
        let attr = Class::new(name, id);
        self.classes.push(attr);
        id
    }
}
