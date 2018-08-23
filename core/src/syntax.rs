
struct RangeNumberSyntax {
    start_number: StartToken,
    hypen: HyphenToken,
    end_number: NumberToken,
}

struct SingleNumberSyntax {
    number: NumberToken,
}

enum NumberSyntax {
    Range(RangeNumberSyntax),
    Single(SingleNumberSyntax),
}

struct UnitsSyntax {
    units: Vec<NumberSyntax>,
}

struct GroupSyntax {
    name: NameToken,
    equals: EqualsToken,
    units: UnitsSyntax,
}

enum SyntaxError {
    NoEqualsAfterIdentifier,
    NoNameBeforeEquals,
}

enum Syntax {
    Plain(UnitsSyntax),
    Groups(Vec<GroupSyntax>),
    PlainWithGroups(UnitsSyntax, Vec<GroupSyntax>),
}

struct SyntaxAnalyzer {
    tokens: Iterator<Token>,
}

mod tests {
    #[test]
    fn invalid_missing_group_name() {
        let input = "=1-10";
        let tokens = ::parse::Tokenizer::new(input).collect();
    }
}
