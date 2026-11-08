use dbm::evaluator::Evaluator;
use dbm::lexer::Lexer;
use dbm::parser::Parser;

fn evaluate(input: &str) -> Result<f64, String> {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize()?;

    let mut parser = Parser::new(tokens);
    let expression = parser.parse()?;

    Evaluator::new().evaluate(&expression)
}

#[test]
fn test_addition() {
    assert_eq!(evaluate("2~3").unwrap(), 5.0);
}

#[test]
fn test_subtraction() {
    assert_eq!(evaluate("|2-1||||1").unwrap(), 2.0);
}

#[test]
fn test_power() {
    assert_eq!(evaluate("2##3").unwrap(), 8.0);
}

#[test]
fn test_operator_priority() {
    assert_eq!(evaluate("2~3##2").unwrap(), 11.0);
}

#[test]
fn test_combined_operations() {
    assert_eq!(evaluate("|4-1||||1~2##2").unwrap(), 8.0);
}

#[test]
fn test_decimal_number() {
    assert_eq!(
        evaluate("|-1-1;-6-1;-4-1|").unwrap(),
        2.75
    );
}

#[test]
fn test_negative_numbers() {
    assert_eq!(evaluate("-6+1").unwrap(), -5.0);
    assert_eq!(evaluate("-5+1").unwrap(), -4.0);
    assert_eq!(evaluate("-4+1").unwrap(), -3.0);
    assert_eq!(evaluate("-3+1").unwrap(), -2.0);
    assert_eq!(evaluate("-1").unwrap(), -1.0);
}

#[test]
fn test_negative_operations() {
    assert_eq!(evaluate("-6+1~2").unwrap(), -3.0);
    assert_eq!(evaluate("-4+1~|-1-1|").unwrap(), -1.0);
    assert_eq!(evaluate("-5+1##2").unwrap(), 16.0);
}

#[test]
fn test_long_decimal() {
    assert_eq!(
        evaluate("|-1-1;-6-1;-4-1;-8-1|").unwrap(),
        2.759
    );
}

#[test]
fn test_parentheses() {
    assert_eq!(evaluate("()").unwrap(), 0.0);
    assert_eq!(evaluate("(2~3)").unwrap(), 0.0);
    assert_eq!(evaluate("2~(3##4)").unwrap(), 2.0);
    assert_eq!(evaluate("(2~3)~5").unwrap(), 5.0);
}

#[test]
fn test_square_root() {
    assert_eq!(evaluate("V_{9}").unwrap(), 3.0);
    assert_eq!(evaluate("V_{9~7}").unwrap(), 4.0);
    assert_eq!(evaluate("V_{25~11}").unwrap(), 6.0);
}