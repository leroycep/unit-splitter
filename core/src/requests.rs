
#[derive(Parser)]
#[grammar = "requests.pest"]
pub struct RequestsParser;

#[cfg(test)]
mod tests {
    use requests::{RequestsParser, Rule};
    use range::Range;

    #[test]
    fn my_one_request() {
        parses_to! {
            parser: RequestsParser,
            input: "A: 32",
            rule: Rule::requests,
            tokens: [
                requests(0, 5, [
                    request(0, 5, [
                        name(0, 1),
                        number(3, 5)
                    ]),
                    EOI(5, 5)
                ])
            ]
        };
    }

    #[test]
    fn repeat_amount() {
        parses_to! {
            parser: RequestsParser,
            input: "A: 32x3",
            rule: Rule::requests,
            tokens: [
                requests(0, 7, [
                    request(0, 7, [
                        name(0, 1),
                        repetition(3, 7, [
                            number(3, 5),
                            number(6, 7)
                        ])
                    ]),
                    EOI(7, 7)
                ])
            ]
        };
    }

    #[test]
    fn many_requests() {
        parses_to! {
            parser: RequestsParser,
            input: "A: 32x3, B: 450,234,4",
            rule: Rule::requests,
            tokens: [
                requests(0, 21, [
                    request(0, 9, [
                        name(0, 1),
                        repetition(3, 7, [
                            number(3, 5),
                            number(6, 7)
                        ])
                    ]),
                    request(9, 21, [
                        name(9, 10),
                        number(12, 15),
                        number(16, 19),
                        number(20, 21)
                    ]),
                    EOI(21, 21)
                ])
            ]
        };
    }
}
