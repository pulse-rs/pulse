use crate::types::Type;
use indexmap::IndexMap;

pub struct Variable {
    pub name: String,
    pub type_: Type,
}

pub struct GlobalContext {
    pub constants: Vec<Variable>,
    pub variables: Vec<Variable>,
}

impl GlobalContext {
    pub fn new() -> Self {
        Self {
            constants: vec![],
            variables: vec![],
        }
    }

    pub fn add_variable(&mut self, name: String, type_: Type) {
        self.variables.push(Variable { name, type_ });
    }

    pub fn add_constant(&mut self, name: String, type_: Type) {
        self.constants.push(Variable { name, type_ });
    }
    
    pub fn get_global_variable(&self, name: &str) -> Option<&Variable> {
        self.variables.iter().find(|v| v.name == name)
    }
}