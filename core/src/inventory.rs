
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
}
