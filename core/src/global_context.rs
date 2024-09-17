use crate::ast::function::{Body, Function};
use crate::ast::{new_id, ID};
use crate::error::error::Error::FunctionAlreadyExists;
use crate::lexer::token::Token;
use crate::types::Type;
use crate::Result;
use indexmap::IndexMap;

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub type_: Type,
    pub shadowing: bool,
}

#[derive(Debug, Clone)]
pub struct GlobalContext {
    pub global_variables: Vec<Variable>,
    pub variables: IndexMap<ID, Variable>,
    pub functions: IndexMap<ID, Function>,
}

impl GlobalContext {
    pub fn new() -> Self {
        Self {
            global_variables: vec![],
            variables: IndexMap::new(),
            functions: IndexMap::new(),
        }
    }

    pub fn add_variable(&mut self, name: String, type_: Type, shadowing: bool, global: bool) -> ID {
        let id = new_id(self.variables.len() as u32);
        self.variables.insert(
            id,
            Variable {
                name,
                type_,
                shadowing,
            },
        );

        if global {
            self.global_variables.push(
                self.variables
                    .get(&id)
                    .expect("Variable was not inserted into the global context")
                    .clone(),
            );
        }

        id
    }
    
    pub fn lookup_var(&self, id: ID) -> Option<&Variable> {
        self.variables.get(&id)
    }

    pub fn get_global_variable(&self, name: &str) -> Option<&Variable> {
        self.global_variables.iter().find(|var| var.name == name)
    }
    
    pub fn lookup_var_id(&self, identifier: &str) -> Option<ID> {
        self.variables
            .iter()
            .find(|(_, var)| var.name == identifier)
            .map(|(id, _)| *id)
    }

    pub fn lookup_function(&self, identifier: &str) -> Option<ID> {
        self.functions
            .iter()
            .find(|(_, function)| function.name == identifier)
            .map(|(id, _)| *id)
    }

    pub fn push_function(&mut self, function: Function) -> ID {
        let id = new_id(self.functions.len() as u32);
        self.functions.insert(id, function);
        id
    }

    pub fn new_function(
        &mut self,
        identifier: Token,
        body: Body,
        parameters: Vec<ID>,
        return_type: Type,
        content: &String,
    ) -> Result<ID> {
        let str_ident = identifier.span.literal.to_string();
        if let Some(id) = self.lookup_function(&str_ident) {
            return Err(FunctionAlreadyExists(
                str_ident,
                identifier.span,
                content.clone(),
            ));
        }

        let function = Function {
            parameters,
            name: str_ident.clone(),
            body,
            return_type,
        };

        Ok(self.push_function(function))
    }
}
