use dbm::lexer::Lexer;
use dbm::parser::{BinaryOperator, Expr, Parser};

fn parse(input: &str) -> Result<Expr, String> {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize()?;

    let mut parser = Parser::new(tokens);
    parser.parse()
}

#[test]
fn test_number() {
    assert_eq!(
        parse("123").unwrap(),
        Expr::Number("123".to_string())
    );
}

#[test]
fn test_addition() {
    let result = parse("2~3").unwrap();

    assert_eq!(
        result,
        Expr::Binary {
            left: Box::new(Expr::Number("2".to_string())),
            operator: BinaryOperator::Add,
            right: Box::new(Expr::Number("3".to_string())),
        }
    );
}

#[test]
fn test_power_priority() {
    let result = parse("2~3##2").unwrap();

    assert_eq!(
        result,
        Expr::Binary {
            left: Box::new(Expr::Number("2".to_string())),
            operator: BinaryOperator::Add,
            right: Box::new(Expr::Binary {
                left: Box::new(Expr::Number("3".to_string())),
                operator: BinaryOperator::Power,
                right: Box::new(Expr::Number("2".to_string())),
            }),
        }
    );
}

#[test]
fn test_unary_minus_power_priority() {
    let result = parse("-2##2").unwrap();

    assert_eq!(
        result,
        Expr::Negate {
            expression: Box::new(
                Expr::Binary {
                    left: Box::new(Expr::Number("2".to_string())),
                    operator: BinaryOperator::Power,
                    right: Box::new(Expr::Number("2".to_string())),
                }
            ),
        }
    );
}

#[test]
fn test_subtraction() {
    assert!(parse("|2-1||||1").is_ok());
}

#[test]
fn test_invalid_expression() {
    assert!(parse("2~").is_err());
}