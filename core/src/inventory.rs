
#[derive(Parser)]
#[grammar = "inventory.pest"]
pub struct InventoryParser;

#[cfg(test)]
mod tests {
    use inventory::{InventoryParser, Rule};

    #[test]
    fn one_group() {
        parses_to! {
            parser: InventoryParser,
            input: "A=1-50",
            rule: Rule::inventory,
            tokens: [
                inventory(0, 6, [
                    group(0, 6, [
                        name(0, 1),
                        range(2, 6, [
                            number(2, 3),
                            hyphen(3, 4),
                            number(4, 6)
                        ])
                    ]),
                    EOI(6, 6)
                ])
            ]
        };
    }

    #[test]
    fn no_groups() {
        parses_to! {
            parser: InventoryParser,
            input: "1-50",
            rule: Rule::inventory,
            tokens: [
                inventory(0, 4, [
                    group(0, 4, [
                        range(0, 4, [
                            number(0, 1),
                            hyphen(1, 2),
                            number(2, 4)
                        ])
                    ]),
                    EOI(4, 4)
                ])
            ]
        };
    }

    #[test]
    fn first_group_is_unnamed() {
        parses_to! {
            parser: InventoryParser,
            input: "1-50, B=51-100, C=101-150",
            rule: Rule::inventory,
            tokens: [
                inventory(0, 25, [
                    group(0, 6, [
                        range(0, 4, [
                             number(0, 1), hyphen(1, 2), number(2, 4)
                        ])
                    ]),
                    group(6, 16, [
                         name(6, 7),
                        range(8, 14, [
                             number(8, 10), hyphen(10, 11), number(11, 14)
                        ])
                    ]),
                    group(16, 25, [
                         name(16, 17),
                        range(18, 25, [
                             number(18, 21), hyphen(21, 22), number(22, 25)
                        ])
                    ]),
                    EOI(25, 25)
                ])
            ]
        };
    }

/*
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

    #[test]
    fn tokenize_first_group_is_unnamed() {
        use ::parse::Token;
        use ::parse::TokenKind::*;
        let source = "1-50, B=51-100, C=101-150";
        let expected = vec![
            Token::number(&source, 0, 1),
            Token::hyphen(&source, 1),
            Token::number(&source, 2, 2),
            Token::name(&source, 6, 1),
            Token::equals(&source, 7),
            Token::number(&source, 8, 2),
            Token::hyphen(&source, 10),
            Token::number(&source, 11, 3),
            Token::name(&source, 16, 1),
            Token::equals(&source, 17),
            Token::number(&source, 18, 3),
            Token::hyphen(&source, 21),
            Token::number(&source, 22, 3),
        ];
        for expected_token in expected.iter() {
            println!("({}, {}): {}", expected_token.start_pos(), expected_token.text().len(), expected_token.text());
        }
        let tokens: Vec<Token> = ::parse::Tokenizer::new(&source).collect();
        assert_eq!(tokens, expected);
    }
*/
}
