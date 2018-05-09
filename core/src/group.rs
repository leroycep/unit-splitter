
use range::Range;

#[derive(PartialEq, Clone, Debug)]
pub struct Group<'name> {
    name: Option<&'name str>,
    ranges: Vec<Range>,
}

impl<'name> Group<'name> {
    pub fn new(name: Option<&'name str>, ranges: Vec<Range>) -> Self {
        Self { name, ranges }
    }

    pub fn name(&self) -> Option<&str> {
        self.name
    }

    pub fn ranges(&self) -> &[Range] {
        &self.ranges
    }

    pub fn count(&self) -> u32 {
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
