
use crate::evaluator::Evaluator;
use crate::formatter::format_number;
use crate::parser::{BinaryOperator, Expr};

pub fn format_expression(expression: &Expr) -> Result<String, String> {
    match expression {
        Expr::Number(_) => {
            let evaluator = Evaluator::new();
            let number = evaluator.evaluate(expression)?;
            format_number(number)
        }

        Expr::Parentheses => Ok("()".to_string()),

        Expr::Grouping { expression } => {
            let inner = format_expression(expression)?;
            Ok(format!("{{{inner}}}"))
        }

        Expr::Negate { expression } => {
            let inner = format_expression(expression)?;
            Ok(format!("-{{{inner}}}"))
        }

        Expr::SquareRoot { expression } => {
            let inner = format_expression(expression)?;
            Ok(format!("V_{{{inner}}}"))
        }

        Expr::Binary {
            left,
            operator,
            right,
        } => {
            let left = format_expression(left)?;
            let right = format_expression(right)?;

            let operator = match operator {
                BinaryOperator::Add => "~",
                BinaryOperator::Subtract => "|||",
                BinaryOperator::Power => "##",
            };

            Ok(format!("{left} {operator} {right}"))
        }
    }
}
