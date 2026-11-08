use dbm::evaluator::Evaluator;
use dbm::formatter::format_number;
use dbm::lexer::Lexer;
use dbm::parser::Parser;

#[test]
fn test_zero_and_one_components_round_trip() {
    let cases = [
        // Простые дроби
        (0.1, "|();1|"),
        (0.5, "|();5|"),
        (1.5, "|1;5|"),
        (2.1, "|-1-1;1|"),

        // Несколько десятичных разрядов
        (3.25, "|-2-1;2;5|"),

        // Нули внутри дробной части
        (3.01, "|-2-1;();1|"),
        (3.001, "|-2-1;();();1|"),

        (-0.5, "-{|();5|}"),
        (-2.75, "-{|-1-1;7;5|}"),
        (-3.01, "-{|-2-1;();1|}"),
    ];

    for (value, expected) in cases {
        let formatted = format_number(value).unwrap();
        assert_eq!(formatted, expected);

        let mut lexer = Lexer::new(&formatted);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let expression = parser.parse().unwrap();
        let result = Evaluator::new()
            .evaluate(&expression)
            .unwrap();

        assert_eq!(result, value);
    }
}

#[test]
fn test_integer_components_round_trip() {
    let cases = [
        (0.0, "()"),
        (1.0, "1"),
        (2.0, "|-1-1|"),
        (3.0, "|-2-1|"),
        (4.0, "|-3-1|"),
        (5.0, "|-4-1|"),
    ];

    for (value, expected) in cases {
        let formatted = format_number(value).unwrap();
        assert_eq!(formatted, expected);

        let mut lexer = Lexer::new(&formatted);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let expression = parser.parse().unwrap();

        let result = Evaluator::new()
            .evaluate(&expression)
            .unwrap();

        assert_eq!(result, value);
    }
}

#[test]
fn test_extreme_number_formatting() {
    // Очень маленькое число.
    let small = 0.0000000001;
    let formatted = format_number(small).unwrap();

    let mut lexer = Lexer::new(&formatted);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let expression = parser.parse().unwrap();

    let result = Evaluator::new()
        .evaluate(&expression)
        .unwrap();

    assert_eq!(result, small);

    // Число за пределами диапазона i64.
    let large = 1e20;
    assert!(format_number(large).is_err());
}