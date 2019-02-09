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

use super::{Module, Boolean, Integer, Float};
use super::class::ClassID;
use super::class_type::TypeID;
use crate::nia::error::syntax;
use crate::nia::parser::{Node, NIL};

type Tuple = Vec<Expression>;
type Inputs = Tuple;
type Outputs = Tuple;

pub enum Expression {
    ClassExpr(ClassID),
    TypeExpr(Option<ClassID>),

    BooleanExpr(Boolean),
    IntegerExpr(Integer),
    FloatExpr(Float),
    StringExpr(String),
    TupleExpr(Tuple),

    ScopeExpr(Inputs, Outputs),
    CallExpr(Box<Expression>, Inputs),

    BranchIfExpr(Box<Expression>),
    BranchElifExpr(Box<Expression>),
    BranchElseExpr,

    UnaryExpr(String, Box<Expression>),
    BinaryExpr(String, Box<Expression>, Box<Expression>),
}

impl Expression {
    fn dummy() -> Self {
        Expression::IntegerExpr(0)
    }
}

impl Module {
    pub fn convert_node_to_expr(&mut self, node: &Node) -> Expression {
        //let name = node.token.clone();
        let name = node.token.trim_end().to_string();
        if node.config.is_string {
            println!("  [string] {:?}", name);
            Expression::StringExpr(name)
        } else if node.config.is_shell_open() {
            println!("  [array] ");
            unimplemented!()
        } else if node.config.is_op {
            println!("  [op] {}", name);
            match name.as_str() {
                ":"     => self.expr_assign(node),
                "!"     => self.expr_call(node),
                ","     => self.expr_tuple(node),
                "?"     => self.expr_type(node),
                "->"    => self.expr_scope(node),
                "-"     => self.expr_minus(node, name),
                "if"    => self.expr_if(node),
                "elif"  => self.expr_elif(node),
                "else"  => self.expr_else(),
                _       => self.expr_binary(node, name),
            }
        } else { self.expr_value(name) }
    }

    fn expr_assign(&mut self, node: &Node) -> Expression {
        let object_opt = self.get_expr_condition(node,node.right != NIL);
        let subject = self.get_expr_force(node);
        match subject {
            Expression::BranchIfExpr(_) => return subject,
            _ => {},
        }
        let cls = self.get_class_id(subject);
        match object_opt {
            Some(object) => match object {
                _ => match object {
                    Expression::TypeExpr(value) => self.set_class_type(cls, value),
                    Expression::ClassExpr(value) => self.set_class_class(cls, value),
                    Expression::BooleanExpr(value) => self.set_class_boolean(cls, value),
                    Expression::IntegerExpr(value) => self.set_class_integer(cls, value),
                    Expression::FloatExpr(value) => self.set_class_float(cls, value),
                    Expression::StringExpr(value) => self.set_class_string(cls, value),
                    Expression::ScopeExpr(inputs, outputs) => self.set_class_scope(cls, inputs, outputs),
                    _ => {},
                },
            },
            None => {},
        }
        println!("    [expected type] {}: {}", (&self.classes[cls]).name, self.get_class_type(cls));
        Expression::ClassExpr(cls)
    }
    fn expr_call(&mut self, node: &Node) -> Expression {
        let object_opt = self.get_expr_condition(node ,node.right != NIL);
        let subject = self.get_expr_force(node);
        let cls = self.get_class_id(subject);
        Expression::CallExpr(Box::new(Expression::ClassExpr(cls)), match object_opt {
            Some(object) => match object {
                Expression::TupleExpr(args) => args,
                _ => vec!(object),
            },
            None => vec!(),
        })
    }
    fn expr_tuple(&mut self, node: &Node) -> Expression {
        let object_opt = self.get_expr_condition(node, node.right != NIL);
        let subject = self.get_expr_force(node);
        match object_opt {
            Some(object) => match object {
                Expression::TupleExpr(mut args) => {
                    args.push(subject);
                    Expression::TupleExpr(args)
                },
                _ => {
                    Expression::TupleExpr(vec!(object, subject))
                },
            },
            None => Expression::TupleExpr(vec!(subject)),
        }
    }
    fn expr_type(&mut self, node: &Node) -> Expression {
        assert_eq!(node.right, NIL);
        let subject_opt = self.get_expr_condition(node, node.left != NIL);
        match subject_opt {
            Some(subject) => match subject {
                Expression::ClassExpr(cls) => Expression::TypeExpr(Some(cls)),
                _ => panic!(),
            }
            None => Expression::TypeExpr(None)
        }
    }
    fn expr_scope(&mut self, node: &Node) -> Expression {
        let object = Self::wrap_vector(self.get_expr_force(node));
        let subject = Self::wrap_vector(self.get_expr_force(node));
        Expression::ScopeExpr(subject, object)
    }
    fn expr_minus(&mut self, node: &Node, name: String) -> Expression {
        let object = self.get_expr_force(node);
        let subject_opt = self.get_expr_condition(node, node.left != NIL);
        match subject_opt {
            Some(subject) => Expression::BinaryExpr(name, Box::new(subject), Box::new(object)),
            None => match object {
                Expression::IntegerExpr(value) => Expression::IntegerExpr(-value),
                Expression::FloatExpr(value) => Expression::FloatExpr(-value),
                _ => Expression::UnaryExpr(name, Box::new(object)),
            },
        }
    }
    fn expr_if(&mut self, node: &Node) -> Expression {
        let condition = self.get_expr_force(node);
        Expression::BranchIfExpr(Box::new(condition))
    }
    fn expr_elif(&mut self, node: &Node) -> Expression {
        let condition = self.get_expr_force(node);
        Expression::BranchElifExpr(Box::new(condition))
    }
    fn expr_else(&self) -> Expression {
        Expression::BranchElseExpr
    }
    fn expr_binary(&mut self, node: &Node, name: String) -> Expression {
        let object = self.get_expr_force(node);
        let subject = self.get_expr_force(node);
        Expression::BinaryExpr(name, Box::new(subject), Box::new(object))
    }
    fn expr_value(&mut self, name: String) -> Expression {
        assert_ne!(name.len(), 0);
        match name.as_str() {
            "yes" => Expression::BooleanExpr(true),
            "no" => Expression::BooleanExpr(false),
            _ => match name.parse::<Integer>() {
                Ok(n) => {
                    println!("  [int] {}", name);
                    Expression::IntegerExpr(n)
                },
                Err(_) => match name.parse::<Float>() {
                    Ok(n) => {
                        println!("  [float] {}", name);
                        Expression::FloatExpr(n)
                    },
                    Err(_) => {
                        println!("  [class] {}", name);
                        Expression::ClassExpr(self.find_class(name))
                    },
                },
            },
        }
    }

    fn wrap_vector(expr: Expression) -> Tuple {
        match expr {
            Expression::TupleExpr(args) => args,
            _ => vec!(expr),
        }
    }

    fn get_expr_force(&mut self, node: &Node) -> Expression {
        match self.stack_expr.pop() {
            Some(e) => e,
            None => {
                syntax::lack_of_parameters(self.path, node);
                Expression::dummy()
            },
        }
    }
    fn get_expr_condition(&mut self, node: &Node, condition: bool) -> Option<Expression> {
        match condition {
            true => Some(self.get_expr_force(node)),
            false => None,
        }
    }
    fn get_class_id(&mut self, expr: Expression) -> ClassID {
        match expr {
            Expression::ClassExpr(cls) => cls,
            _ => panic!(),
        }
    }
    fn set_class_type(&mut self, cls: ClassID, value: Option<ClassID>) {
        let t = match value {
            Some(id) => {
                println!("    [assert type] {}: {}", &self.classes[cls].name, &self.classes[id].name);
                TypeID::from(&self.classes[id])
            },
            None => {
                println!("    [assert type] {}: ?", &self.classes[cls].name);
                TypeID::Any
            }
        };
        let cls = &mut self.classes[cls];
        cls.set_type(t)
    }
    fn set_class_class(&mut self, cls: ClassID, value: ClassID) {
        let value_cls = &self.classes[value];
        if ! value_cls.has_value() {
            println!("    [assert type] {}: {}", &self.classes[cls].name, &self.classes[value].name);
            let value_type = value_cls.type_id.clone();
            let cls = &mut self.classes[cls];
            cls.set_type(value_type);
            return
        }
        println!("    [assert class] {}: {}", &self.classes[cls].name, &self.classes[value].name);
        let value_type = (&self.classes[value]).type_id.clone();
        let cls = &mut self.classes[cls];
        cls.set_class(value, value_type)
    }
    fn set_class_boolean(&mut self, cls: ClassID, value: Boolean) {
        let cls = &mut self.classes[cls];
        println!("    [assert bool] {}: {}", cls.name, value);
        cls.set_boolean(value)
    }
    fn set_class_integer(&mut self, cls: ClassID, value: Integer) {
        let cls = &mut self.classes[cls];
        println!("    [assert int] {}: {}", cls.name, value);
        cls.set_integer(value)
    }
    fn set_class_float(&mut self, cls: ClassID, value: Float) {
        let cls = &mut self.classes[cls];
        println!("    [assert float] {}: {}", cls.name, value);
        cls.set_float(value)
    }
    fn set_class_string(&mut self, cls: ClassID, value: String) {
        let cls = &mut self.classes[cls];
        println!("    [assert str] {}: {}", cls.name, value);
        cls.set_string(value)
    }
    fn set_class_scope(&mut self, cls: ClassID, inputs: Inputs, outputs: Outputs) {
        fn assert_class(expr: &Expression) -> ClassID {
            match expr {
                Expression::ClassExpr(id) => *id,
                _ => panic!(),
            }
        }
        let cls = &mut self.classes[cls];
        println!("    [assert scope] {}: [{}] -> [{}]", cls.name, inputs.len(), outputs.len());
        let inputs = inputs.iter().map(assert_class).collect();
        let outputs = outputs.iter().map(assert_class).collect();
        cls.set_scope(inputs, outputs)
    }
}