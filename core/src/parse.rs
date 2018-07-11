include!(concat!(env!("OUT_DIR"), "/notation.rs"));

use ::interval_tree::IntervalTreeNode;
use std::collections::HashSet;

pub fn find_overlaps(ranges: &[Range]) -> HashSet<(Range, Range)> {
    assert!(ranges.len() > 0);
    let mut tree = IntervalTreeNode::new(ranges[0].clone());
    // Stores the overlaps that will be returned fromthis function
    let mut overlaps_total = HashSet::new();
    // Stores the overlaps temporarily for each range
    let mut overlaps = vec![];
    for range in ranges.iter().skip(1) {
        overlaps.clear();
        tree.overlap_search(range, &mut overlaps);
        for conflict in overlaps.iter() {
            overlaps_total.insert((range.clone(), conflict.clone()));
        }
        tree.insert(range.clone());
    }

    overlaps_total
}

#[cfg(test)]
mod tests {
    use range::Range;
    use group::Group;

    #[test]
    fn one_group() {
        let expected = Group::new("A".into(), vec![Range::new(1, 50)]);
        assert_eq!(::parse::parse_units("A=1-50"), Ok(vec![expected]));
    }

    #[test]
    fn no_groups() {
        let expected = Group::new("".into(), vec![Range::new(1, 50)]);
        assert_eq!(::parse::parse_units("1-50"), Ok(vec![expected]));
    }

    #[test]
    fn first_group_is_unnamed() {
        let expected = vec![
            Group::new("".into(), vec![Range::new(1, 50)]),
            Group::new("B".into(), vec![Range::new(51, 100)]),
            Group::new("C".into(), vec![Range::new(101, 150)]),
        ];
        assert_eq!(::parse::parse_units("1-50, B=51-100, C=101-150"), Ok(expected));
    }

    #[test]
    fn multiple_groups() {
        let expected = vec![
            Group::new("A".into(), vec![Range::new(1, 50)]),
            Group::new("B".into(), vec![Range::new(51, 100)]),
            Group::new("C".into(), vec![Range::new(101, 150)]),
        ];
        assert_eq!(::parse::parse_units("A=1-50, B=51-100, C=101-150"), Ok(expected));
    }

    #[test]
    fn multiple_ranges() {
        let expected = Group::new("".into(), vec![
                                 Range::new(1, 7),
                                 Range::new(8, 8),
                                 Range::new(10, 10),
                                 Range::new(11, 50),
                                 ]);
        assert_eq!(::parse::parse_units("1-7,8,10,11-50"), Ok(vec![expected]));
    }

    #[test]
    fn ambiguous_group_unit_name() {
        let expected = vec![
            Group::new("995N".into(), vec![Range::new(1, 50)]),
            Group::new("998N".into(), vec![Range::new(51, 100)]),
        ];
        assert_eq!(::parse::parse_units("995N=1-50, 998N=51-100"), Ok(expected));
    }

    #[test]
    fn group_name_is_a_number() {
        let expected = vec![
            Group::new("995".into(), vec![Range::new(1, 50)]),
            Group::new("998".into(), vec![Range::new(51, 100)]),
        ];
        assert_eq!(::parse::parse_units("995=1-50, 998=51-100"), Ok(expected));
    }

    #[test]
    fn overlapping_ranges_are_detected() {
        let ranges = vec![Range::new(1, 50), Range::new(51, 100), Range::new(50,50)];
        assert_eq!(::parse::find_overlaps(&ranges).len(), 1);
    }

    #[test]
    fn geeksforgeeks_example() {
        use std::collections::HashSet;
        let ranges = vec![ Range::new(1, 5), Range::new(3, 7), Range::new(2, 6), Range::new(10, 15), Range::new(5, 6), Range::new(4, 100) ];

        let mut conflicts = HashSet::new();
        conflicts.insert((Range::new(3, 7), Range::new(1, 5)));
        conflicts.insert((Range::new(2, 6), Range::new(1, 5)));
        conflicts.insert((Range::new(2, 6), Range::new(3, 7)));
        conflicts.insert((Range::new(5, 6), Range::new(1, 5)));
        conflicts.insert((Range::new(5, 6), Range::new(3, 7)));
        conflicts.insert((Range::new(5, 6), Range::new(2, 6)));
        conflicts.insert((Range::new(4, 100), Range::new(1, 5)));
        conflicts.insert((Range::new(4, 100), Range::new(3, 7)));
        conflicts.insert((Range::new(4, 100), Range::new(2, 6)));
        conflicts.insert((Range::new(4, 100), Range::new(10, 15)));
        conflicts.insert((Range::new(4, 100), Range::new(5, 6)));

        let overlaps = ::parse::find_overlaps(&ranges);

        println!("Expected Conflicts:");
        for c in conflicts.iter() {
            println!("\t[{},{}] should conflict with [{},{}]", c.0.first(), c.0.last(), c.1.first(), c.1.last());
        }

        println!("\nConflicts:");
        for c in overlaps.iter() {
            println!("\t[{},{}] conflicts with [{},{}]", c.0.first(), c.0.last(), c.1.first(), c.1.last());
        }

        assert_eq!(overlaps, conflicts);
    }
}
