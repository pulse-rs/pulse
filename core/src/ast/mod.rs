use crate::ast::item::{Item, ItemKind};
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
    // pub stmts: IndexMap<ID>,
    // pub exprs: IndexMap<ID>,
}

impl Ast {
    pub fn new() -> Self {
        Self {
            items: IndexMap::new(),
            // stmts: IndexMap::new(),
            // exprs: IndexMap::new(),
        }
    }

    pub fn new_item(&mut self, kind: ItemKind) -> &Item {
        let id = new_id(self.items.len() as u32);
        let item = Item::new(kind, id);
        self.items.insert(id, item);

        &self.items.get(&id).unwrap()
    }
}
