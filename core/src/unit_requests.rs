
use std::collections::HashMap;
use slab::Slab;

type TestId = usize;
type GroupId = usize;
type Amount = usize;

pub struct UnitRequests {
    tests: Slab<String>,
    amounts: HashMap<(TestId, GroupId), Amount>, // <( GroupID, TestID ), Amount >
}

impl UnitRequests {
    pub fn new() -> Self {
        Self {
            tests: Slab::new(),
            amounts: HashMap::new(),
        }
    }

    pub fn add_test(&mut self, name: &str) -> usize {
        self.tests.insert(name.into())
    }

    pub fn get_amount(&self, test_id: usize, group_id: usize) -> usize {
        self.amounts.get(&(test_id, group_id)).map(|e| *e).unwrap_or(0)
    }

    pub fn set_amount(&mut self, test_id: usize, group_id: usize, amount: usize) {
        self.amounts.insert((test_id, group_id), amount);
    }

    pub fn remove_test(&mut self, test_id: usize) {
        self.tests.remove(test_id);
        self.amounts.retain(
            |(test_id_key, _group_id), _value| *test_id_key != test_id
        );
    }

}

#[cfg(test)]
mod tests {
    // TODO: add tests?
}
