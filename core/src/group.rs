use range::Range;
use std::fmt;

#[derive(PartialEq, Clone, Debug)]
pub struct Group {
    name: String,
    ranges: Vec<Range>,
}

impl Group {
    pub fn new(name: String, ranges: Vec<Range>) -> Self {
        Self { name, ranges }
    }

    pub fn with_ranges(&self, ranges: Vec<Range>) -> Self {
        Self {
            name: self.name.clone(),
            ranges,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn ranges(&self) -> &[Range] {
        &self.ranges
    }

    pub fn count(&self) -> u32 {
        self.ranges.iter().map(|r| r.count()).sum()
    }
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.name != "" {
            write!(f, "{}=", self.name)?;
        }
        let mut need_comma = false;
        for range in &self.ranges {
            if need_comma {
                write!(f, ",{}", range)?;
            } else {
                write!(f, "{}", range)?;
                need_comma = true;
            }
        }
        Ok(())
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

    #[test]
    fn format_no_name() {
        let group = Group::new("".into(), vec![Range::new(1, 50), Range::num(61)]);
        assert_eq!(format!("{}", group), "1-50,61");
    }

    #[test]
    fn format_with_name() {
        let group = Group::new("A".into(), vec![Range::new(1, 50), Range::num(61)]);
        assert_eq!(format!("{}", group), "A=1-50,61");
    }
}
