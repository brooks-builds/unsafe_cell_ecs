use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use crate::data::Data;

#[derive(Default, Debug)]
pub struct Resources {
    pub data: HashMap<String, Data>,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: &str, data: Data) {
        self.data.insert(name.to_string(), data);
    }

    pub fn get(&self, name: &str) -> Option<&Data> {
        self.data.get(name)
    }
}
