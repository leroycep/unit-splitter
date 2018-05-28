
use range::Range;

#[derive(PartialEq, Clone, Debug)]
pub struct Group {
    name: String,
    ranges: Vec<Range>,
}

impl Group {
    pub fn new(name: String, ranges: Vec<Range>) -> Self {
        Self { name, ranges }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn ranges(&self) -> &[Range] {
        &self.ranges
    }

    pub fn count(&self) -> usize {
        self.ranges.iter().map(|r| r.count()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use range::Range;

    #[test]
    fn it_works() {
        let range = Range::new(1, 50);
        let _group = Group::new("A".into(), vec![range]);
    }
}
