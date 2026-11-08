
use crate::parser::{BinaryOperator, Expr};

pub struct Evaluator;

impl Evaluator {
    pub fn new() -> Self {
        Self
    }

    pub fn evaluate(&self, expression: &Expr) -> Result<f64, String> {
        match expression {
            Expr::Number(value) => self.evaluate_number(value),

            Expr::Parentheses => Ok(0.0),

            Expr::Grouping { expression } => self.evaluate(expression),

            Expr::Negate { expression } => {
                let value = self.evaluate(expression)?;
                Ok(-value)
            }

            Expr::SquareRoot { expression } => {
                let value = self.evaluate(expression)?;

                if value < 0.0 {
                    return Err(
                        "Cannot calculate square root of a negative number".to_string()
                    );
                }

                Ok(value.sqrt())
            }

            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.evaluate(left)?;
                let right = self.evaluate(right)?;

                match operator {
                    BinaryOperator::Add => Ok(left + right),
                    BinaryOperator::Subtract => Ok(left - right),
                    BinaryOperator::Power => Ok(left.powf(right)),
                }
            }
        }
    }

    fn evaluate_number(&self, value: &str) -> Result<f64, String> {
        if value.starts_with('|') && value.ends_with('|') {
            let inner = &value[1..value.len() - 1];
            let parts: Vec<&str> = inner.split(';').collect();

            if parts.is_empty() {
                return Err("Empty DBM number".to_string());
            }

            let integer = Self::decode_component(parts[0])?;
            let mut result = integer;

            for (index, part) in parts.iter().skip(1).enumerate() {
                let digit = Self::decode_component(part)?;

                if !(0.0..=9.0).contains(&digit)
                    || digit.fract() != 0.0
                {
                    return Err(format!(
                        "Invalid decimal component: {part}"
                    ));
                }

                result += digit / 10_f64.powi((index + 1) as i32);
            }

            return Ok(result);
        }

        if value.starts_with('-') && value.contains('+') {
            let (first, second) = value
                .split_once('+')
                .ok_or_else(|| format!("Invalid negative number: {value}"))?;

            let first: f64 = first
                .parse()
                .map_err(|_| format!("Invalid negative number: {value}"))?;

            let second: f64 = second
                .parse()
                .map_err(|_| format!("Invalid negative number: {value}"))?;

            return Ok(first + second);
        }

        value.parse::<f64>()
            .map_err(|_| format!("Invalid number: {value}"))
    }

    fn decode_component(value: &str) -> Result<f64, String> {
        if value == "()" {
            return Ok(0.0);
        }

        // Обычная запись компоненты: 1, 2, 3 и т.д.
        if let Ok(number) = value.parse::<f64>() {
            return Ok(number);
        }

        // Специальная DBM-запись: -1-1, -2-1 и т.д.
        let normalized = value.strip_prefix('-').unwrap_or(value);

        let (first, second) = normalized
            .split_once('-')
            .ok_or_else(|| format!("Invalid DBM component: {value}"))?;

        let first: f64 = first
            .parse()
            .map_err(|_| format!("Invalid DBM component: {value}"))?;

        let second: f64 = second
            .parse()
            .map_err(|_| format!("Invalid DBM component: {value}"))?;

        Ok(first + second)
    }
}
