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

use super::class::{Class, ClassID};

const STR_ANY: &'static str = "?";
const STR_SCOPE: &'static str = "->";
const STR_BOOL: &'static str = "bool";
const STR_INT: &'static str = "int";
const STR_FLOAT: &'static str = "float";
const STR_STR: &'static str = "str";

type Tuple = Vec<TypeID>;
type Inputs = Tuple;
type Outputs = Tuple;

pub enum TypeID {
    Any,
    Class(ClassID),
    Scope(Inputs, Outputs),
    Boolean,
    Integer,
    Float,
    String,
}

impl TypeID {
    pub fn void_scope(len_inputs: usize, len_outputs: usize) -> Self {
        TypeID::Scope(
            vec!(TypeID::Any; len_inputs),
            vec!(TypeID::Any; len_outputs),
        )
    }
    pub fn new_scope(inputs: Inputs, outputs: Outputs) -> Self {
        TypeID::Scope(
            inputs,
            outputs,
        )
    }
    pub fn from(cls: &Class) -> Self {
        match cls.name.as_str() {
            STR_BOOL    => TypeID::Boolean,
            STR_INT     => TypeID::Integer,
            STR_FLOAT   => TypeID::Float,
            STR_STR     => TypeID::String,
            _           => TypeID::Class(cls.current),
        }
    }
    pub fn as_str(&self) -> String {
        String::from(match self {
            TypeID::Any         => STR_ANY,
            TypeID::Boolean     => STR_BOOL,
            TypeID::Scope(_, _) => STR_SCOPE,
            TypeID::Integer     => STR_INT,
            TypeID::Float       => STR_FLOAT,
            TypeID::String      => STR_STR,
            _                   => "__class__",
        })
    }
}

impl Clone for TypeID {
    fn clone(&self) -> Self {
        match self {
            TypeID::Any         => TypeID::Any,
            TypeID::Class(id)   => TypeID::Class(*id),
            TypeID::Scope(i, o)   => TypeID::Scope(i.clone(), o.clone()),
            TypeID::Boolean     => TypeID::Boolean,
            TypeID::Integer     => TypeID::Integer,
            TypeID::Float       => TypeID::Float,
            TypeID::String      => TypeID::String,
        }
    }
}
