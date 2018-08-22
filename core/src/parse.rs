include!(concat!(env!("OUT_DIR"), "/notation.rs"));

use ::interval_tree::IntervalTreeNode;
use std::collections::HashSet;


#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Token {
    start: usize,
    len: usize,
    kind: TokenKind,
}

impl Token {
    pub fn new(start: usize, len: usize, kind: TokenKind) -> Self {
        Self {
            start,
            len,
            kind,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum TokenKind {
    Equals,
    Hyphen,
    Number,
    Identifier,
}

struct Tokenizer {
    source: Vec<(usize, char)>,
    pos: usize,
}

impl Tokenizer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.char_indices().collect(),
            pos: 0,
        }
    }

    pub fn get_next_token(&mut self) -> Option<Token> {
        loop {
            match self.next_char() {
                None => return None,
                Some((pos, '=')) => return Some(Token::new(pos, 1, TokenKind::Equals)),
                Some((pos, '-')) => return Some(Token::new(pos, 1, TokenKind::Hyphen)),
                Some((_, c)) if is_whitespace(c) => continue,
                Some((start_pos, c)) if c.is_digit(10) => {
                    let mut c;
                    let mut is_number = true;
                    loop {
                        c = match self.peek_char() {
                            None => break,
                            Some((_pos, c)) => c
                        };
                        if c.is_digit(10) { }
                        else if is_whitespace(c) { break }
                        else if is_hyphen(c) { break }
                        else if is_identifier(c) { is_number = false; }
                        else { /* TODO: Report error, unexpected character */ }
                        self.next_char(); // Consume character if loop has not broke yet
                    }
                    let len = self.pos - start_pos;
                    return Some(if is_number {
                        Token::new(start_pos, len, TokenKind::Number)
                    } else {
                        Token::new(start_pos, len, TokenKind::Identifier)
                    });
                },
                Some((start_pos, c)) if is_identifier(c) => {
                    let mut c;
                    loop {
                        c = match self.peek_char() {
                            None => break,
                            Some((_pos, c)) => c
                        };
                        if is_identifier(c) { }
                        else if is_whitespace(c) { break }
                        else if is_equals(c) { break }
                        else { /* TODO: Report error, unexpected character */ }
                        self.next_char(); // Consume character if loop has not broke yet
                    }
                    let len = self.pos - start_pos;
                    return Some(Token::new(start_pos, len, TokenKind::Identifier));
                },
                Some((_pos, _c)) => {
                    // TODO: Report error, unexpected character
                }
            }
        }
    }

    //fn get_number(&mut self)

    fn next_char(&mut self) -> Option<(usize, char)> {
        let result = self.source.get(self.pos).map(|(p, c)| (*p, *c));
        if result.is_some() {
            self.pos += 1;
        }
        result
    }

    fn peek_char(&self) -> Option<(usize, char)> {
        self.source.get(self.pos).map(|(p, c)| (*p, *c))
    }
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.get_next_token()
    }
}

pub fn is_equals(c: char) -> bool {
    c == '='
}

pub fn is_hyphen(c: char) -> bool {
    c == '-'
}

pub fn is_whitespace(c: char) -> bool {
    c == ' '
    || c == '\n'
    || c == '\r'
    || c == '\t'
    || c == ','
}

pub fn is_identifier(c: char) -> bool {
    c.is_alphanumeric() || c == '-'
}

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

    #[test]
    fn tokenize_first_group_is_unnamed() {
        use ::parse::Token;
        use ::parse::TokenKind::*;
        let text = "1-50, B=51-100, C=101-150";
        let expected = vec![
            Token::new(0, 1, Number),
            Token::new(1, 1, Hyphen),
            Token::new(2, 2, Number),
            Token::new(6, 1, Identifier),
            Token::new(7, 1, Equals),
            Token::new(8, 2, Number),
            Token::new(10, 1, Hyphen),
            Token::new(11, 3, Number),
            Token::new(16, 1, Identifier),
            Token::new(17, 1, Equals),
            Token::new(18, 3, Number),
            Token::new(21, 1, Hyphen),
            Token::new(22, 3, Number),
        ];
        let tokens: Vec<Token> = ::parse::Tokenizer::new(text).collect();
        assert_eq!(tokens, expected);
    }

}
