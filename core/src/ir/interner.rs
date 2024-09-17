use rustc_hash::FxHashMap;
use std::{collections::HashMap, mem};

pub struct Interner {
    pub map: FxHashMap<&'static str, StrId>,
    pub vec: Vec<&'static str>,
    pub buf: String,
    pub full: Vec<String>,
}

impl Interner {
    pub fn new() -> Interner {
        Interner::with_capacity(2)
    }

    pub fn with_capacity(cap: usize) -> Interner {
        let cap = cap.next_power_of_two();
        Interner {
            map: HashMap::default(),
            vec: Vec::new(),
            buf: String::with_capacity(cap),
            full: Vec::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<StrId> {
        self.map.get(name).copied()
    }

    pub fn intern(&mut self, name: &str) -> StrId {
        if let Some(&id) = self.map.get(name) {
            return id;
        }
        let name = unsafe { self.alloc(name) };
        let id = StrId(self.map.len() as u32);
        self.map.insert(name, id);
        self.vec.push(name);
        id
    }

    pub fn lookup(&self, id: StrId) -> &str {
        self.vec[id.0 as usize]
    }

    unsafe fn alloc(&mut self, name: &str) -> &'static str {
        let cap = self.buf.capacity();
        if cap < self.buf.len() + name.len() {
            let new_cap = (cap.max(name.len()) + 1).next_power_of_two();
            let new_buf = String::with_capacity(new_cap);
            let old_buf = mem::replace(&mut self.buf, new_buf);
            self.full.push(old_buf);
        }
        let interned = {
            let start = self.buf.len();
            self.buf.push_str(name);
            &self.buf[start..]
        };
        &*(interned as *const str)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StrId(u32);

impl StrId {
    pub const DUMMY: StrId = StrId(u32::MAX);
}
