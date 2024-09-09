use crate::ast::function::FunctionDeclaration;
use crate::ast::ID;

#[derive(Debug, Clone)]
pub enum ItemKind {
    Stmt(ID),
    Function//(FunctionDeclaration),
}

#[derive(Debug, Clone)]
pub struct Item {
    pub id: ID,
    pub kind: ItemKind,
}

impl Item {
    pub fn new(kind: ItemKind, id: ID) -> Self {
        Self { kind, id }
    }
}
