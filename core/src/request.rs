#[derive(Clone, Debug, PartialEq)]
pub struct Request {
    name: String,
    amounts: Vec<u32>,
}

impl Request {
    pub fn new(name: String, amounts: Vec<u32>) -> Self {
        Self { name, amounts }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn amounts(&self) -> &[u32] {
        &self.amounts
    }
}
