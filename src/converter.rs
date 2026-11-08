use crate::evaluator::Evaluator;
use crate::formatter::format_number;
use crate::expression_formatter::format_expression;
use crate::lexer::Lexer;
use crate::parser::Parser;

pub struct Converter;

impl Converter {
    pub fn convert_expression(input: &str) -> Result<String, String> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize()?;

        let mut parser = Parser::new(tokens);
        let expression = parser.parse()?;

        let evaluator = Evaluator::new();
        let result = evaluator.evaluate(&expression)?;

        if !result.is_finite() {
            return Err("Cannot format a non-finite result".to_string());
        }

        let formatted_expression = format_expression(&expression)?;

        let marker = if result < 0.0 {
            "-неа"
        } else {
            "неа"
        };

        let formatted_result = result.abs().to_string();

        Ok(format!(
            "{{{formatted_expression}}} {marker} = {{{formatted_result}}}"
        ))
    }
}

pub fn convert(value: f64) -> Result<String, String> {
    format_number(value)
}

pub fn convert_expression(input: &str) -> Result<String, String> {
    Converter::convert_expression(input)
}