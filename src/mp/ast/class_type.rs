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

use crate::mp::ast::class::{Class, ClassID};

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
            "bool" => TypeID::Boolean,
            "int" => TypeID::Integer,
            "float" => TypeID::Float,
            "str" => TypeID::String,
            _ => TypeID::Class(cls.current),
        }
    }
    pub fn as_str(&self) -> String {
        String::from(match self {
            TypeID::Any     => "?",
            TypeID::Boolean => "bool",
            TypeID::Integer => "int",
            TypeID::Float   => "float",
            TypeID::String  => "str",
            _               => "_",
        })
    }
    pub fn clone(&self) -> TypeID {
        match self {
            TypeID::Any         => TypeID::Any,
            TypeID::Class(id)   => TypeID::Class(id.clone()),
            TypeID::Boolean     => TypeID::Boolean,
            TypeID::Integer     => TypeID::Integer,
            TypeID::Float       => TypeID::Float,
            TypeID::String      => TypeID::String,
        }
    }
}
