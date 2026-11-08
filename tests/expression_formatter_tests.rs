use dbm::expression_formatter::format_expression;
use dbm::lexer::Lexer;
use dbm::parser::Parser;

fn format(input: &str) -> Result<String, String> {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize()?;

    let mut parser = Parser::new(tokens);
    let expression = parser.parse()?;

    format_expression(&expression)
}

#[test]
fn test_number() {
    assert_eq!(format("5").unwrap(), "|-4-1|");
}

#[test]
fn test_parentheses() {
    assert_eq!(format("()").unwrap(), "()");
}

#[test]
fn test_grouping() {
    assert_eq!(format("{2~3}").unwrap(), "{|-1-1| ~ |-2-1|}");
}

#[test]
fn test_negate() {
    assert_eq!(format("-5").unwrap(), "-{|-4-1|}");
}

#[test]
fn test_square_root() {
    assert_eq!(format("V_{9}").unwrap(), "V_{|-8-1|}");
}

#[test]
fn test_binary_operators() {
    assert_eq!(format("2~3").unwrap(), "|-1-1| ~ |-2-1|");
    assert_eq!(format("3|||2").unwrap(), "|-2-1| ||| |-1-1|");
    assert_eq!(format("2##3").unwrap(), "|-1-1| ## |-2-1|");
}