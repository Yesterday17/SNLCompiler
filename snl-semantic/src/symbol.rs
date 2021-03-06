use std::collections::HashMap;
use std::cmp::min;

#[derive(Debug)]
pub enum Symbol {
    Variable(String),
    Procedure(Vec<String>),
    Type(String),
}

pub struct SymbolTable<T> {
    inner: Vec<HashMap<String, T>>
}

impl<T> Default for SymbolTable<T> {
    fn default() -> Self {
        SymbolTable::new()
    }
}

impl<T> SymbolTable<T> {
    pub fn new() -> Self {
        Self { inner: vec![HashMap::new()] }
    }

    pub fn level(&self) -> usize {
        self.inner.len()
    }

    pub fn step_in(&mut self) {
        self.inner.push(Default::default());
    }

    pub fn step_out(&mut self) -> HashMap<String, T> {
        self.inner.pop().unwrap()
    }

    pub fn query(&self, key: &str) -> Option<&T> {
        self.query_at(key, self.inner.len())
    }

    pub fn query_at(&self, key: &str, level: usize) -> Option<&T> {
        if self.inner.len() == 0 {
            return None;
        }

        let mut level = (min(level, self.inner.len()) - 1) as i32;
        while level >= 0 {
            let table = self.inner.get(level as usize).unwrap();
            if let Some(value) = table.get(key) {
                return Some(value);
            }
            level -= 1;
        }
        None
    }

    pub fn insert(&mut self, key: String, value: T) {
        self.inner.last_mut().unwrap().insert(key, value);
    }

    pub fn has_own_property(&self, key: &str) -> bool {
        self.inner.last().unwrap().contains_key(key)
    }
}

impl SymbolTable<Symbol> {
    pub fn query_type(&self, ty: &str) -> Option<&str> {
        let ty = if ty.starts_with("#") {
            &ty[1..]
        } else {
            ty
        };
        match self.query(ty) {
            Some(Symbol::Type(ty)) => {
                if ty.starts_with("#") {
                    self.query_type(&ty)
                } else {
                    Some(ty.as_str())
                }
            }
            _ => None,
        }
    }
}