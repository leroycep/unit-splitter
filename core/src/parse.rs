include!(concat!(env!("OUT_DIR"), "/notation.rs"));

use ::interval_tree::IntervalTreeNode;
use std::collections::HashSet;
use std::str::CharIndices;
use std::iter::Peekable;


#[derive(Eq, PartialEq, Debug, Clone)]
pub struct TokenMeta<'source> {
    start: usize,
    text: &'source str,
}

impl<'source> TokenMeta<'source> {
    pub fn new(source: &'source str, start: usize, len: usize) -> Self {
        let text = &source[start..start+len];
        Self {
            start,
            text,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct EqualsToken<'source> {
    meta: TokenMeta<'source>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct HyphenToken<'source> {
    meta: TokenMeta<'source>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct NumberToken<'source> {
    meta: TokenMeta<'source>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct NameToken<'source> {
    meta: TokenMeta<'source>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Token<'source> {
    Equals(EqualsToken<'source>),
    Hyphen(HyphenToken<'source>),
    Number(NumberToken<'source>),
    Name(NameToken<'source>),
}

impl<'source> Token<'source> {
    pub fn equals(source: &'source str, start: usize) -> Self {
        let meta = TokenMeta::new(source, start, 1);
        Token::Equals(EqualsToken { meta })
    }

    pub fn hyphen(source: &'source str, start: usize) -> Self {
        let meta = TokenMeta::new(source, start, 1);
        Token::Hyphen(HyphenToken { meta })
    }

    pub fn number(source: &'source str, start: usize, len: usize) -> Self {
        let meta = TokenMeta::new(source, start, len);
        Token::Number(NumberToken { meta })
    }

    pub fn name(source: &'source str, start: usize, len: usize) -> Self {
        let meta = TokenMeta::new(source, start, len);
        Token::Name(NameToken { meta })
    }

    pub fn start_pos(&self) -> usize {
        use parse::Token::*;
        match self {
            Equals(token) => token.meta.start,
            Hyphen(token) => token.meta.start,
            Number(token) => token.meta.start,
            Name(token) => token.meta.start,
        }
    }

    pub fn text(&self) -> &'source str {
        use parse::Token::*;
        match self {
            Equals(token) => token.meta.text,
            Hyphen(token) => token.meta.text,
            Number(token) => token.meta.text,
            Name(token) => token.meta.text,
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

struct Tokenizer<'source> {
    source_str: &'source str,
    source: Peekable<CharIndices<'source>>,
}

impl<'source> Tokenizer<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            source_str: source,
            source: source.char_indices().peekable(),
        }
    }

    pub fn get_next_token(&mut self) -> Option<Token<'source>> {
        loop {
            match self.next_char() {
                None => return None,
                Some((pos, '=')) => return Some(Token::equals(self.source_str, pos)),
                Some((pos, '-')) => return Some(Token::hyphen(self.source_str, pos)),
                Some((_, c)) if is_whitespace(c) => continue,
                Some((start_pos, c)) if c.is_digit(10) => {
                    let mut pos = start_pos;
                    let mut c = c;
                    let mut is_number = true;
                    loop {
                        let next_pair = match self.peek_char() {
                            None => {
                                pos += 1;
                                break;
                            },
                            Some((pos, c)) => (pos, c),
                        };
                        pos = next_pair.0;
                        c = next_pair.1;
                        if c.is_digit(10) { }
                        else if is_whitespace(c) { break }
                        else if is_hyphen(c) { break }
                        else if is_identifier(c) { is_number = false; }
                        else { /* TODO: Report error, unexpected character */ }
                        self.next_char(); // Consume character if loop has not broke yet
                    }
                    let len = pos - start_pos;
                    return Some(if is_number {
                        Token::number(self.source_str, start_pos, len)
                    } else {
                        Token::name(self.source_str, start_pos, len)
                    });
                },
                Some((start_pos, c)) if is_identifier(c) => {
                    let mut pos = start_pos;
                    let mut c;
                    loop {
                        let next_pair = match self.peek_char() {
                            None => {
                                pos += 1;
                                break;
                            },
                            Some((pos, c)) => (pos, c),
                        };
                        pos = next_pair.0;
                        c = next_pair.1;
                        if is_identifier(c) { }
                        else if is_whitespace(c) { break }
                        else if is_equals(c) { break }
                        else { /* TODO: Report error, unexpected character */ }
                        self.next_char(); // Consume character if loop has not broke yet
                    }
                    let len = pos - start_pos;
                    return Some(Token::name(self.source_str, start_pos, len));
                },
                Some((_pos, _c)) => {
                    // TODO: Report error, unexpected character
                }
            }
        }
    }

    //fn get_number(&mut self)

    fn next_char(&mut self) -> Option<(usize, char)> {
        self.source.next().map(|(p, c)| (p.clone(), c.clone()))
    }

    fn peek_char(&mut self) -> Option<(usize, char)> {
        self.source.peek().map(|(p, c)| (p.clone(), c.clone()))
    }
}

impl<'source> Iterator for Tokenizer<'source> {
    type Item = Token<'source>;

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

}
