/* ------------------------------------------------------------
    Universal Task-Distributed Language
    Project.Github: "https://github.com/kerryeon/nia_rust"
---------------------------------------------------------------
    Author:
        Name: "kerryeon"
        Email: "besqer996@gnu.ac.kr"
        Github: "https://github.com/kerryeon"
    Generated:
        Date: "2019-01-29"
------------------------------------------------------------ */

use super::class::{Class, ClassID, Scope};
use super::class_type::TypeID;
use super::Module;
use super::expression::Expression;
use crate::nia::config;
use crate::nia::parser::Node;

pub struct Line {
    expr: Option<Expression>,
    indent: config::NumIndent,
}

impl Line {
    fn new(indent: config::NumIndent) -> Self {
        Self {
            expr: None,
            indent,
        }
    }
    fn from(cls: ClassID, indent: config::NumIndent) -> Self {
        Self {
            expr: Some(Expression::ClassExpr(cls)),
            indent,
        }
    }
}

impl Module {
    pub fn new(path: &'static str) -> Self {
        Self {
            path,
            attrs: vec!(),
            classes: vec!(),
            using: vec!(),
            scopes: vec!(),
            stack_expr: vec!(),
        }
    }
    pub fn begin_line(&mut self, num_indent: config::NumIndent) {
        println!("[new line] {} indents", num_indent);
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
            match &line.expr {
                Some(expr) => match expr {
                    Expression::ClassExpr(attr) => {
                        self.update_type_for_class(*attr);
                    },
                    _ => panic!(),
                },
                None => {},
            }

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
                            match &line.expr {
                                Some(expr) => match expr {
                                    Expression::ClassExpr(cls) => self.attrs.push(*cls),
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
                        Expression::ClassExpr(attr) => {
                            self.add_attr_in_class(cls, attr);
                        },
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
                match self.find_class_in_class(attr, &name) {
                    Some(id) => return id,
                    None => continue,
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
        cls.add_attr(attr)
    }
    fn update_type_for_class(&mut self, id: ClassID) {
        let cls = &self.classes[id];
        let (inputs, outputs) = match cls.get_scope() {
            Some(scope) => self.collect_type_scope(scope),
            None => return,
        };
        let cls = &mut self.classes[id];
        cls.set_scope_type(inputs, outputs)
    }

    fn collect_type_tuple(&self, tuple: &Vec<ClassID>) -> Vec<TypeID> {
        tuple.iter().map(|id| self.classes[*id].type_id.clone()).collect()
    }
    fn collect_type_scope(&self, scope: &Scope) -> (Vec<TypeID>, Vec<TypeID>) {
        let (inputs, outputs) = scope.unzip();
        (self.collect_type_tuple(inputs), self.collect_type_tuple(outputs))
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
            match self.find_class_in_class(attr, &name) {
                Some(id) => return id,
                None => continue,
            }
        }
        self.new_class(name)
    }
    fn find_class_in_class(&self, cls: &Class, name: &String) -> Option<ClassID> {
        println!("dfd! {} vs {}", cls.name, name);
        if cls.name == *name {
            return Some(cls.current)
        }
        let (inputs, outputs) = match cls.get_scope() {
            Some(scope) => self.collect_type_scope(scope),
            None => return None,
        };
        None
    }
    fn new_class(&mut self, name: String) -> ClassID {
        println!("    [new class] {}", name);
        let id: ClassID = self.classes.len();
        let attr = Class::new(name, id);
        self.classes.push(attr);
        id
    }
}
