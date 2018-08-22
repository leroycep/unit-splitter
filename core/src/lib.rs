
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate slab;

pub mod group;
pub mod range;
pub mod unit_requests;
pub mod parse;
pub mod split;
pub mod interval_tree;


use group::Group;
use std::collections::HashMap;

struct Procedure {
    id: usize,
    name: String,
}

struct Request {
    group_id: usize,
    procedure_id: usize,
    amount: usize,
}

pub struct Core {
    groups: Vec<Group>,
    procedures: HashMap<usize, String>,
    requests: Vec<Request>,
    // Metadata
    procedure_next_id: usize,
}

impl Core {
    pub fn new() -> Self {
        Core {
            groups: vec![],
            procedures: HashMap::new(),
            requests: vec![],
            procedure_next_id: 0,
        }
    }

    pub fn set_groups(&mut self, groups: Vec<Group>) {
        self.groups = groups;
    }

    pub fn add_procedure(&mut self) -> usize {
        let id = self.procedure_next_id;
        let name = String::new();
        self.procedure_next_id += 1;
        self.procedures.insert(id, name);
        id
    }

    pub fn get_procedure_name(&self, id: usize) -> Option<&String> {
        self.procedures.get(&id)
    }

    pub fn set_procedure_name(&mut self, id: usize, text: String) {
        if self.procedures.get(&id).is_none() {
            return;
        }
        self.procedures.insert(id, text);
    }
}
