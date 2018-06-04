include!(concat!(env!("OUT_DIR"), "/notation.rs"));

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
}
