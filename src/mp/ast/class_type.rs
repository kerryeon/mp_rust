/* ------------------------------------------------------------
    Machine Pseudo-Code
    Project.Github: !(https://github.com/kerryeon/mp_rust)
---------------------------------------------------------------
    Author:
        Name: kerryeon
        Email: besqer996@gnu.ac.kr
        Github: !(https://github.com/kerryeon)
    Generated:
        Date: 2019-01-29
------------------------------------------------------------ */

use super::class::{Class, ClassID};

const STR_BOOL: &'static str = "bool";
const STR_INT: &'static str = "int";
const STR_FLOAT: &'static str = "float";
const STR_STR: &'static str = "str";

pub enum TypeID {
    Any,
    Class(ClassID),
    Boolean,
    Integer,
    Float,
    String,
}

impl TypeID {
    pub fn from(cls: &Class) -> TypeID {
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
            TypeID::Any     => "?",
            TypeID::Boolean => STR_BOOL,
            TypeID::Integer => STR_INT,
            TypeID::Float   => STR_FLOAT,
            TypeID::String  => STR_STR,
            _               => "__class__",
        })
    }
    pub fn clone(&self) -> TypeID {
        match self {
            TypeID::Any         => TypeID::Any,
            TypeID::Class(id)   => TypeID::Class(*id),
            TypeID::Boolean     => TypeID::Boolean,
            TypeID::Integer     => TypeID::Integer,
            TypeID::Float       => TypeID::Float,
            TypeID::String      => TypeID::String,
        }
    }
}
