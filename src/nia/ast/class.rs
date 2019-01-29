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

use super::{Boolean, Integer, Float};
use super::class_type::TypeID;

pub type ClassID = usize;

enum ClassType {
    Any,
    Class(ClassID),
    ValueBoolean(Boolean),
    ValueInteger(Integer),
    ValueFloat(Float),
    ValueString(String),
}

pub struct Class {
    pub name: String,
    pub type_id: TypeID,
    value: Box<ClassType>,

    pub current: ClassID,
    pub parent: Option<ClassID>,
    pub attrs: Vec<ClassID>,
}

impl Class {
    pub fn new(name: String, id: ClassID) -> Class {
        Class {
            name,
            type_id: TypeID::Class(id),
            value: Box::new(ClassType::Any),
            current: id,
            parent: None,
            attrs: vec!(),
        }
    }

    pub fn set_type(&mut self, value: TypeID) {
        self.type_id = value
    }
    pub fn set_class(&mut self, value: ClassID, value_type: TypeID) {
        self.type_id = value_type;
        self.value = Box::new(ClassType::Class(value))
    }
    pub fn set_boolean(&mut self, value: Boolean) {
        self.type_id = TypeID::Boolean;
        self.value = Box::new(ClassType::ValueBoolean(value))
    }
    pub fn set_integer(&mut self, value: Integer) {
        self.type_id = TypeID::Integer;
        self.value = Box::new(ClassType::ValueInteger(value))
    }
    pub fn set_float(&mut self, value: Float) {
        self.type_id = TypeID::Float;
        self.value = Box::new(ClassType::ValueFloat(value))
    }
    pub fn set_string(&mut self, value: String) {
        self.type_id = TypeID::String;
        self.value = Box::new(ClassType::ValueString(value))
    }
}