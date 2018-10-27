use crate::request::Request;
use pest::Parser;

#[derive(Parser)]
#[grammar = "requests.pest"]
pub struct RequestsParser;

pub type RequestsParseResult = Result<Vec<Request>, Vec<RequestsParseError>>;

pub fn parse(input: &str) -> RequestsParseResult {
    let mut parse = RequestsParser::parse(Rule::requests, input).map_err(|x| vec![x.into()])?;
    let requests = parse
        .next()
        .expect("If there is no input, SyntaxError is returned in the above statement");

    let mut requests_data = vec![];
    for request in requests.into_inner() {
        match request.as_rule() {
            Rule::request => {
                let mut inner = request.into_inner();
                let name = inner.next().unwrap().as_str().into();

                let mut amounts = vec![];

                for amount_parse in inner {
                    match amount_parse.as_rule() {
                        Rule::number => {
                            let num = amount_parse.as_str().parse().unwrap();
                            amounts.push(num);
                        }
                        Rule::repetition => {
                            let mut inner = amount_parse.into_inner();
                            let num = inner.next().unwrap().as_str().parse().unwrap();
                            let num_repeat = inner.next().unwrap().as_str().parse().unwrap();
                            for _i in 0..num_repeat {
                                amounts.push(num);
                            }
                        }
                        _ => unreachable!(),
                    }
                }

                requests_data.push(Request::new(name, amounts));
            }
            Rule::EOI => {}
            _ => unreachable!(),
        }
    }

    Ok(requests_data)
}

#[derive(Fail, Debug, PartialEq)]
pub enum RequestsParseError {
    #[fail(display = "Invalid syntax: {}", _0)]
    Syntax(#[cause] ::pest::error::Error<Rule>),
}

impl From<::pest::error::Error<Rule>> for RequestsParseError {
    fn from(error: ::pest::error::Error<Rule>) -> Self {
        RequestsParseError::Syntax(error)
    }
}

#[cfg(test)]
mod tests {
    use request::Request;
    use requests::{parse, RequestsParser, Rule};

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

    #[test]
    fn many_requests_to_data() {
        let result = parse("A: 32x3, B: 450,234,4");
        assert_eq!(
            result,
            Ok(vec![
                Request::new("A".to_string(), vec![32, 32, 32]),
                Request::new("B".to_string(), vec![450, 234, 4]),
            ])
        );
    }
}
