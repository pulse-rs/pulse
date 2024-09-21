use crate::ast::function::Function;
use crate::ast::ID;
use crate::global_context::GlobalContext;
use crate::types::Type;

#[derive(Debug, Clone)]
pub struct LocalScope {
    locals: Vec<ID>,
    function: Option<ID>,
}

impl LocalScope {
    pub fn new(function: Option<ID>) -> Self {
        LocalScope {
            locals: Vec::new(),
            function,
        }
    }

    pub fn add_local(&mut self, local: ID) {
        self.locals.push(local);
    }
}

#[derive(Debug)]
pub struct Scopes<'a> {
    pub local: Vec<LocalScope>,
    pub global: &'a mut GlobalContext,
}

impl<'a> Scopes<'a> {
    pub fn new(ctx: &'a mut GlobalContext) -> Self {
        Scopes {
            local: vec![],
            global: ctx,
        }
    }

    pub fn push_scope(&mut self, function: Option<ID>) {
        self.local.push(LocalScope::new(function));
    }

    pub fn pop_scope(&mut self) {
        self.local.pop();
    }

    pub fn add_local(&mut self, local: ID) {
        if let Some(scope) = self.local.last_mut() {
            scope.add_local(local);
        }
    }

    pub fn in_scope(&self) -> bool {
        !self.local.is_empty()
    }

    pub fn new_var(&mut self, name: String, type_: Type) -> ID {
        let is_global = self.in_scope();
        let id = {
            let shadowing = if let Some(scope) = self.local.last_mut() {
                scope.locals.iter().any(|local| {
                    let var = self.global.variables.get(local).unwrap();
                    var.name == name
                })
            } else {
                false
            };

            self.global.add_variable(name, type_, shadowing, !is_global)
        };

        if is_global {
            self.add_local(id);
        }

        id
    }

    pub fn lookup_var(&self, name: &str) -> Option<ID> {
        for scope in self.local.iter().rev() {
            for local in scope.locals.iter().rev() {
                let var = self.global.variables.get(local)?;
                if var.name == name {
                    return Some(*local);
                }
            }
        }

        self.global.lookup_var_id(name)
    }

    pub fn current_function(&self) -> Option<&Function> {
        self.local
            .iter()
            .rev()
            .filter_map(|scope| scope.function)
            .next()
            .and_then(|function| self.global.functions.get(&function))
    }
}
