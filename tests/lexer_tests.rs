
use dbm::lexer::{Lexer, TokenKind};

#[test]
fn test_basic_expression() {
    let mut lexer = Lexer::new("|2-1|~1");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Number("|2-1|".into()));
    assert_eq!(tokens[1].kind, TokenKind::Plus);
    assert_eq!(tokens[2].kind, TokenKind::Number("1".into()));
    assert_eq!(tokens[3].kind, TokenKind::EOF);
}

#[test]
fn test_subtraction() {
    let mut lexer = Lexer::new("|2-1||||1");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Number("|2-1|".into()));
    assert_eq!(tokens[1].kind, TokenKind::Minus);
    assert_eq!(tokens[2].kind, TokenKind::Number("1".into()));
}

#[test]
fn test_power() {
    let mut lexer = Lexer::new("2##3");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Number("2".into()));
    assert_eq!(tokens[1].kind, TokenKind::Power);
    assert_eq!(tokens[2].kind, TokenKind::Number("3".into()));
}

#[test]
fn test_decimal_number() {
    let mut lexer = Lexer::new("|-1-1;-6-1;-4-1|");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(
        tokens[0].kind,
        TokenKind::Number("|-1-1;-6-1;-4-1|".into())
    );
}

#[test]
fn test_negative_numbers() {
    let mut lexer = Lexer::new("-6+1");
    let mut tokens = lexer.tokenize().unwrap();

    assert_eq!(
        tokens[0].kind,
        TokenKind::Number("-6+1".to_string())
    );
    lexer = Lexer::new("-5+1");
    tokens = lexer.tokenize().unwrap();
    assert_eq!(
        tokens[0].kind,
        TokenKind::Number("-5+1".into())
    );
    lexer = Lexer::new("-4+1");
    tokens = lexer.tokenize().unwrap();
    assert_eq!(
        tokens[0].kind,
        TokenKind::Number("-4+1".to_string())
    );
}