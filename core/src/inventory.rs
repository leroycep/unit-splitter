
use group::Group;
use range::Range;
use pest::Parser;
use ::interval_tree::IntervalTree;

#[derive(Parser)]
#[grammar = "inventory.pest"]
pub struct InventoryParser;

/// An owned version of `pest::Span` that can be put into error types easily.
#[derive(PartialEq, Clone, Debug)]
pub struct OwnedSpan {
    /// The start of this text in the original string
    start: usize,
    /// The end of this text in the original string
    end: usize,
    /// A copy of the text that this Span represents
    text: String,
}

impl OwnedSpan {
    pub fn new(start: usize, end: usize, text: String) -> Self {
        Self {
            start, end, text
        }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

impl<'i> From<pest::Span<'i>> for OwnedSpan {
    fn from(span: pest::Span) -> Self {
        Self {
            text: span.as_str().into(),
            start: span.start(),
            end: span.end(),
        }
    }
}

#[derive(Fail, Debug, PartialEq)]
pub enum InventoryParseError {
    #[fail(display = "Invalid syntax: {}", _0)]
    Syntax(::pest::error::Error<Rule>),

    #[fail(display = "Overlapping unit numbers: {:?}", overlaps)]
    OverlappingUnits {
        overlaps: Vec<(OwnedSpan, OwnedSpan)>,
    },

    #[fail(display = "Duplicate group names: {:?}", duplicates)]
    DuplicateGroups {
        duplicates: Vec<(OwnedSpan, OwnedSpan)>,
    },
}

impl From<pest::error::Error<Rule>> for InventoryParseError {
    fn from(error: pest::error::Error<Rule>) -> Self {
        InventoryParseError::Syntax(error)
    }
}

pub fn parse(input: &str) -> Result<Vec<Group>, InventoryParseError> {
    let mut parse = InventoryParser::parse(Rule::inventory, input)?;
    let inventory = parse.next().expect("If there is no input, SyntaxError is returned in the above statement");
    let mut groups = vec![];
    let mut overlapping_ranges = vec![];
    for group in inventory.into_inner() {
        match group.as_rule() {
            Rule::group => {
                let mut inner = group.into_inner();
                let first = inner.next().unwrap();

                let mut ranges = vec![];
                let mut interval_tree = IntervalTree::new();

                let name;
                if first.as_rule() == Rule::name {
                    name = String::from(first.as_str());
                } else {
                    name = String::new();
                    let range = parse_ranges_from_rules(first.clone());
                    interval_tree.insert(range.clone(), first.as_span());
                    ranges.push(range);
                }

                for pair in inner {
                    let range = parse_ranges_from_rules(pair.clone());

                    let mut overlaps = vec![];
                    interval_tree.overlap_search(&range, &mut overlaps);
                    for (_overlapping_range, overlapping_span) in overlaps {
                        overlapping_ranges.push((pair.as_span().into(), overlapping_span.into()));
                    }

                    interval_tree.insert(range.clone(), pair.as_span());

                    ranges.push(range);
                }

                groups.push(Group::new(name, ranges));
            }
            Rule::EOI => {}
            _ => unreachable!(),
        }
    }

    if overlapping_ranges.len() > 0 {
        Err(InventoryParseError::OverlappingUnits { overlaps: overlapping_ranges })
    } else {
        Ok(groups)
    }
}

/// Parses a Pair that is of `Rule::range` or `Rule::number` into a Range
fn parse_ranges_from_rules(pair: pest::iterators::Pair<Rule>) -> Range {
    match pair.as_rule() {
        Rule::number => {
            Range::num(pair.as_str().parse().expect("The number rule should be parseable by rust number parser"))
        }
        Rule::range => {
            let mut inner = pair.into_inner();
            let first = inner.next().expect("Rule::range must have two numbers").as_str().parse().expect("Number rule should be parseable by rust number parser");
            let last = inner.next().expect("Rule::range must have two numbers").as_str().parse().expect("Number rule should be parseable by rust number parser");
            Range::new(first, last)
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use inventory::{InventoryParser, Rule, InventoryParseError, OwnedSpan, parse};
    use group::Group;
    use range::Range;

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
                             number(0, 1), number(2, 4)
                        ])
                    ]),
                    group(6, 16, [
                         name(6, 7),
                        range(8, 14, [
                             number(8, 10), number(11, 14)
                        ])
                    ]),
                    group(16, 25, [
                         name(16, 17),
                        range(18, 25, [
                             number(18, 21), number(22, 25)
                        ])
                    ]),
                    EOI(25, 25)
                ])
            ]
        };
    }

    #[test]
    fn multiple_groups() {
        parses_to! {
            parser: InventoryParser,
            input: "A=1-50, B=51-100, C=101-150",
            rule: Rule::inventory,
            tokens: [
                inventory(0, 27, [
                    group(0, 8, [
                         name(0, 1),
                        range(2, 6, [
                             number(2, 3), number(4, 6)
                        ])
                    ]),
                    group(8, 18, [
                         name(8, 9),
                        range(10, 16, [
                             number(10, 12), number(13, 16)
                        ])
                    ]),
                    group(18, 27, [
                         name(18, 19),
                        range(20, 27, [
                             number(20, 23), number(24, 27)
                        ])
                    ]),
                    EOI(27, 27)
                ])
            ]
        };
    }

    #[test]
    fn multiple_ranges() {
        parses_to! {
            parser: InventoryParser,
            input: "1-7,8,10,11-50",
            rule: Rule::inventory,
            tokens: [
                inventory(0, 14, [
                    group(0, 14, [
                        range(0, 3, [number(0, 1), number(2, 3)]),
                        number(4, 5),
                        number(6, 8),
                        range(9, 14, [number(9, 11), number(12, 14)]),
                    ]),
                    EOI(14, 14)
                ])
            ]
        };
    }

    #[test]
    fn ambiguous_group_unit_name() {
        parses_to! {
            parser: InventoryParser,
            input: "995N=1-50, 998N=51-100",
            rule: Rule::inventory,
            tokens: [
                inventory(0, 22, [
                    group(0, 11, [
                        name(0, 4),
                        range(5, 9, [number(5, 6), number(7, 9)]),
                    ]),
                    group(11, 22, [
                        name(11, 15),
                        range(16, 22, [number(16, 18), number(19, 22)]),
                    ]),
                    EOI(22, 22)
                ])
            ]
        };
    }

    #[test]
    fn group_name_is_a_number() {
        parses_to! {
            parser: InventoryParser,
            input: "995=1-50, 998=51-100",
            rule: Rule::inventory,
            tokens: [
                inventory(0, 20, [
                    group(0, 10, [
                        name(0, 3),
                        range(4, 8, [number(4, 5), number(6, 8)]),
                    ]),
                    group(10, 20, [
                        name(10, 13),
                        range(14, 20, [number(14, 16), number(17, 20)]),
                    ]),
                    EOI(20, 20)
                ])
            ]
        };
    }

    #[test]
    fn parse_units_into_types() {
        let result = parse("1-10,12");
        let expected = vec![
            Group::new("".to_string(), vec![
                Range::new(1, 10),
                Range::num(12),
            ])
        ];

        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn overlapping_ranges() {
        let result = parse("1-10,5");

        match result {
            Ok(_) => panic!("Overlapping ranges should throw an error."),
            Err(InventoryParseError::Syntax(_)) => panic!("Overlapping ranges are not a syntax error."),
            Err(InventoryParseError::DuplicateGroups { duplicates: _ }) => panic!("Overlapping ranges are not a duplicate groups error."),
            Err(InventoryParseError::OverlappingUnits { overlaps }) => {
                assert!(overlaps.len() == 1);
                let overlap = &overlaps[0];
                let expected = &(OwnedSpan::new(5, 6, "5".into()), OwnedSpan::new(0, 4, "1-10".into()));

                assert_eq!(overlap, expected);
            }
        }
    }

    #[test]
    fn duplicate_groups() {
        let result = parse("A=1-10, A=11-20");

        match result {
            Ok(_) => panic!("Duplicated groups should throw an error."),
            Err(InventoryParseError::Syntax(_)) => panic!("Duplicated groups are not a syntax error."),
            Err(InventoryParseError::OverlappingUnits { overlaps: _ }) => panic!("Duplicated groups are not a overlapping units error."),
            Err(InventoryParseError::DuplicateGroups { duplicates }) => {
                assert!(duplicates.len() == 1);
                let duplicate = &duplicates[0];

                assert_eq!(duplicate.0.start(), 0);
                assert_eq!(duplicate.0.end(), 1);

                assert_eq!(duplicate.1.start(), 8);
                assert_eq!(duplicate.1.end(), 9);
            }
        }
    }
}
