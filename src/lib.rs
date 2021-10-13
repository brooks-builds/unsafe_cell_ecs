use std::cell::UnsafeCell;

use data::Data;
use resources::Resources;

pub mod data;
pub mod resources;

#[derive(Debug)]
pub struct World {
    pub resources: Resources,
}

impl World {
    pub fn new() -> Self {
        Self {
            resources: Resources::new(),
        }
    }

    pub fn insert_resource(&mut self, name: &str, data: Data) {
        self.resources.insert(name, data);
    }

    pub fn get_resource(&self, name: &str) -> Option<&Data> {
        self.resources.get(name)
    }
}
