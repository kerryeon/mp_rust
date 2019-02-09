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
    Scope(Scope),
    ValueBoolean(Boolean),
    ValueInteger(Integer),
    ValueFloat(Float),
    ValueString(String),
}

pub struct Class {
    pub name: String,
    pub type_id: TypeID,
    value: ClassType,

    pub current: ClassID,
    pub parent: Option<ClassID>,
    pub attrs: Vec<ClassID>,
}

type Tuple = Vec<ClassID>;
type Inputs = Tuple;
type Outputs = Tuple;
pub struct Scope {
    inputs: Inputs,
    outputs: Outputs,
}

impl Class {
    pub fn new(name: String, id: ClassID) -> Self {
        Self {
            name,
            type_id: TypeID::Class(id),
            value: ClassType::Any,
            current: id,
            parent: None,
            attrs: vec!(),
        }
    }

    pub fn has_value(&self) -> bool {
        match &self.value {
            ClassType::Any => false,
            _ => true,
        }
    }
    pub fn get_scope(&self) -> Option<&Scope> {
        match &self.value {
            ClassType::Scope(scope) => Some(scope),
            _ => None,
        }
    }

    pub fn set_type(&mut self, value: TypeID) {
        self.type_id = value
    }
    pub fn set_class(&mut self, value: ClassID, value_type: TypeID) {
        self.type_id = value_type;
        self.value = ClassType::Class(value)
    }
    pub fn set_boolean(&mut self, value: Boolean) {
        self.type_id = TypeID::Boolean;
        self.value = ClassType::ValueBoolean(value)
    }
    pub fn set_integer(&mut self, value: Integer) {
        self.type_id = TypeID::Integer;
        self.value = ClassType::ValueInteger(value)
    }
    pub fn set_float(&mut self, value: Float) {
        self.type_id = TypeID::Float;
        self.value = ClassType::ValueFloat(value)
    }
    pub fn set_string(&mut self, value: String) {
        self.type_id = TypeID::String;
        self.value = ClassType::ValueString(value)
    }
    pub fn set_scope(&mut self, inputs: Inputs, outputs: Outputs) {
        self.type_id = TypeID::void_scope(inputs.len(), outputs.len());
        self.value = ClassType::Scope(Scope::new(inputs, outputs))
    }

    pub fn set_scope_type(&mut self, inputs: Vec<TypeID>, outputs: Vec<TypeID>) {
        self.type_id = TypeID::new_scope(inputs, outputs)
    }

    pub fn add_attr(&mut self, attr: ClassID) {
        self.attrs.push(attr)
    }
}

impl Scope {
    pub fn new(inputs: Inputs, outputs: Outputs) -> Self {
        Self {
            inputs,
            outputs,
        }
    }

    pub fn unzip(&self) -> (&Inputs, &Outputs) {
        (&self.inputs, &self.outputs)
    }
}