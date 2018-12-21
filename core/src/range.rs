use std::fmt;

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

    pub fn split(&self, amount: u32) -> (Option<Self>, Option<Self>, u32) {
        if amount == 0 {
            (None, Some(self.clone()), 0)
        } else if amount >= self.count() {
            (Some(self.clone()), None, amount - self.count())
        } else {
            let other_first = self.first + amount;
            let this_last = other_first - 1;
            (
                Some(Range::new(self.first, this_last)),
                Some(Range::new(other_first, self.last)),
                0,
            )
        }
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        !(self.last < other.first || self.first > other.last)
    }
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.first == self.last {
            write!(f, "{}", self.first)
        } else {
            write!(f, "{}-{}", self.first, self.last)
        }
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
            (Some(Range::new(1, 5)), Some(Range::new(6, 10)), 0)
        );
    }

    #[test]
    fn format_single() {
        let range = Range::new(696, 696);
        assert_eq!(format!("{}", range), "696");
    }

    #[test]
    fn format_range() {
        let range = Range::new(1, 10);
        assert_eq!(format!("{}", range), "1-10");
    }
}
