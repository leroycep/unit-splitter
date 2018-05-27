include!(concat!(env!("OUT_DIR"), "/notation.rs"));

#[cfg(test)]
mod tests {
    use range::Range;
    use group::Group;

    #[test]
    fn one_group() {
        let expected = Group::new(Some("A".into()), vec![Range::new(1, 50)]);
        assert_eq!(::parse::parse_units("A=1-50"), Ok(vec![expected]));
    }

    #[test]
    fn no_groups() {
        let expected = Group::new(None, vec![Range::new(1, 50)]);
        assert_eq!(::parse::parse_units("1-50"), Ok(vec![expected]));
    }

    #[test]
    fn multiple_groups() {
        let expected = vec![
            Group::new(Some("A".into()), vec![Range::new(1, 50)]),
            Group::new(Some("B".into()), vec![Range::new(51, 100)]),
            Group::new(Some("C".into()), vec![Range::new(101, 150)]),
        ];
        assert_eq!(::parse::parse_units("A=1-50, B=51-100, C=101-150"), Ok(expected));
    }

    #[test]
    fn multiple_ranges() {
        let expected = Group::new(None, vec![
                                 Range::new(1, 7),
                                 Range::new(8, 8),
                                 Range::new(10, 10),
                                 Range::new(11, 50),
                                 ]);
        assert_eq!(::parse::parse_units("1-7,8,10,11-50"), Ok(vec![expected]));
    }
}
