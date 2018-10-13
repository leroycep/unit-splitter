#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Range {
    first: u32,
    last: u32,
}

impl Range {
    pub fn new(first: u32, last: u32) -> Self {
        Self { first, last }
    }

    /// A convience function for a range of size one
    pub fn num(num: u32) -> Self {
        Self {
            first: num,
            last: num,
        }
    }

    pub fn first(&self) -> u32 {
        self.first
    }

    pub fn last(&self) -> u32 {
        self.last
    }

    pub fn count(&self) -> u32 {
        self.last - self.first + 1
    }

    pub fn write_to_string(&self, string: &mut String) {
        use std::fmt::Write;
        if self.first == self.last {
            write!(string, "{}", self.first);
        } else {
            write!(string, "{}-{}", self.first, self.last);
        }
    }

    pub fn split(&self, amount: u32) -> (Self, Option<Self>, u32) {
        assert!(amount != 0);
        if amount >= self.count() {
            (self.clone(), None, amount - self.count())
        } else {
            let other_first = self.first + amount;
            let this_last = other_first - 1;
            (
                Range::new(self.first, this_last),
                Some(Range::new(other_first, self.last)),
                0,
            )
        }
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        !(self.last < other.first || self.first > other.last)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let range = Range::new(1, 50);
        assert_eq!(range.count(), 50);
    }

    #[test]
    fn count_of_one() {
        let range = Range::new(1, 1);
        assert_eq!(range.count(), 1);
    }

    #[test]
    fn split() {
        let range = Range::new(1, 10);
        assert_eq!(
            range.split(5),
            (Range::new(1, 5), Some(Range::new(6, 10)), 0)
        );
    }
}
