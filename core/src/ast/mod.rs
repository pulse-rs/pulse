use crate::ast::function::{Body, FunctionParameter, FunctionType, TypeAnnotation};
use crate::ast::item::{Item, ItemKind};
use crate::ast::stmt::{LetStmt, Stmt, StmtKind};
use crate::lexer::token::Token;
use crate::Result;
use indexmap::IndexMap;

pub mod function;
pub mod item;
pub mod position;
pub mod span;
pub mod stmt;

pub type ID = u32;

pub fn new_id(mut last_id: ID) -> ID {
    last_id += 1;
    last_id
}

#[derive(Debug, Clone)]
pub struct Ast {
    pub items: IndexMap<ID, Item>,
    pub stmts: IndexMap<ID, Stmt>,
    // pub exprs: IndexMap<ID>,
}

impl Ast {
    pub fn new() -> Self {
        Self {
            items: IndexMap::new(),
            stmts: IndexMap::new(),
            // exprs: IndexMap::new(),
        }
    }

    pub fn new_item(&mut self, kind: ItemKind) -> &Item {
        let id = new_id(self.items.len() as u32);
        let item = Item::new(kind, id);
        self.items.insert(id, item);

        &self.items.get(&id).unwrap()
    }

    pub fn new_stmt(&mut self, kind: StmtKind) -> ID {
        let id = new_id(self.items.len() as u32);
        let stmt = Stmt::new(kind, id);
        self.stmts.insert(stmt.id, stmt);

        id
    }

    pub fn let_stmt(
        &mut self,
        identifier: Token,
        initializer: u32,
        type_annotation: Option<TypeAnnotation>,
    ) -> ID {
        let var_id = new_id(self.stmts.len() as u32);

        let let_stmt = StmtKind::Let(LetStmt {
            identifier,
            initializer,
            type_annotation,
            variable_id: var_id,
        });

        self.new_stmt(let_stmt)
    }

    pub fn new_func_item(
        &mut self,
        func_keyword: Token,
        identifier: Token,
        parameters: Vec<FunctionParameter>,
        body: Body,
        return_type: Option<FunctionType>,
        function_id: ID,
    ) -> Result<&Item> {
        let func_decl = ItemKind::Function(function::FunctionDeclaration {
            func_keyword,
            identifier,
            parameters,
            body,
            return_type,
            id: function_id,
        });

        Ok(self.new_item(func_decl))
    }
}
