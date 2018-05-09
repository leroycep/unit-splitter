
#[derive(Debug, Fail)]
pub enum RangeError {
    #[fail(display = "last unit is before first unit")]
    LastIsBeforeFirst,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Range {
    first: u32,
    last: u32,
}

impl Range {
    pub fn new(first: u32, last: u32) -> Result<Self, RangeError> {
        if first <= last {
            Ok(Self { first, last })
        } else {
            Err(RangeError::LastIsBeforeFirst)
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
}
