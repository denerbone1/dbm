
use crate::lexer::{Token, TokenKind};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(String),
    Parentheses,
    Grouping {
        expression: Box<Expr>,
    },
    Negate {
        expression: Box<Expr>,
    },
    SquareRoot {
        expression: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: BinaryOperator,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Power,
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
    }

    pub fn parse(&mut self) -> Result<Expr, String> {
        let expression = self.parse_expression()?;

        match self.current() {
            Some(token) if token.kind == TokenKind::EOF => Ok(expression),
            Some(token) => Err(format!(
                "Unexpected token at position {}: {:?}",
                token.position, token.kind
            )),
            None => Ok(expression),
        }
    }

    // Lowest priority: addition
    fn parse_expression(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_subtraction()?;

        while let Some(token) = self.current() {
            if token.kind != TokenKind::Plus {
                break;
            }

            self.advance();

            let right = self.parse_subtraction()?;

            left = Expr::Binary {
                left: Box::new(left),
                operator: BinaryOperator::Add,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    // Medium priority: subtraction
    fn parse_subtraction(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_power()?;

        while let Some(token) = self.current() {
            if token.kind != TokenKind::Minus {
                break;
            }

            self.advance();

            let right = self.parse_power()?;

            left = Expr::Binary {
                left: Box::new(left),
                operator: BinaryOperator::Subtract,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    // Highest priority: exponentiation
    // Right-associative: 2##3##2 = 2##(3##2)
    fn parse_power(&mut self) -> Result<Expr, String> {
        let left = self.parse_unary()?;

        if let Some(token) = self.current() {
            if token.kind == TokenKind::Power {
                self.advance();

                let right = self.parse_power()?;

                return Ok(Expr::Binary {
                    left: Box::new(left),
                    operator: BinaryOperator::Power,
                    right: Box::new(right),
                });
            }
        }

        Ok(left)
    }

    // Numbers and other basic values
    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.current() {
            Some(token) => match &token.kind {
                TokenKind::Number(value) => {
                    let expression = Expr::Number(value.clone());
                    self.advance();
                    Ok(expression)
                }

                TokenKind::LeftParen => {
                    self.advance();

                    if let Some(token) = self.current() {
                        if token.kind == TokenKind::RightParen {
                            self.advance();
                            return Ok(Expr::Parentheses);
                        }
                    }

                    let expression = self.parse_expression()?;

                    match self.current() {
                        Some(token) if token.kind == TokenKind::RightParen => {
                            self.advance();
                            
                            let _ = expression;
                            Ok(Expr::Parentheses)
                        }

                        Some(token) => Err(format!(
                            "Expected closing parenthesis at position {}, found {:?}",
                            token.position, token.kind
                        )),

                        None => Err("Expected closing parenthesis".to_string()),
                    }
                }

                TokenKind::LeftBrace => {
                    self.advance();

                    let expression = self.parse_expression()?;

                    match self.current() {
                        Some(token) if token.kind == TokenKind::RightBrace => {
                            self.advance();

                            Ok(Expr::Grouping {
                                expression: Box::new(expression),
                            })
                        }

                        Some(token) => Err(format!(
                            "Expected closing brace at position {}, found {:?}",
                            token.position, token.kind
                        )),

                        None => Err("Expected closing brace".to_string()),
                    }
                }

                TokenKind::SquareRoot => {
                    self.advance();

                    match self.current() {
                        Some(token) if token.kind == TokenKind::LeftBrace => {
                            self.advance();

                            let expression = self.parse_expression()?;

                            match self.current() {
                                Some(token) if token.kind == TokenKind::RightBrace => {
                                    self.advance();

                                    Ok(Expr::SquareRoot {
                                        expression: Box::new(expression),
                                    })
                                }

                                Some(token) => Err(format!(
                                    "Expected closing brace at position {}, found {:?}",
                                    token.position, token.kind
                                )),

                                None => Err("Expected closing brace".to_string()),
                            }
                        }

                        Some(token) => Err(format!(
                            "Expected opening brace after square root at position {}, found {:?}",
                            token.position, token.kind
                        )),

                        None => Err("Expected opening brace after square root".to_string()),
                    }
                }

                _ => Err(format!(
                    "Expected a number or opening parenthesis at position {}, found {:?}",
                    token.position, token.kind
                )),
            },

            None => Err("Unexpected end of expression".to_string()),
        }
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        if let Some(token) = self.current() {
            if token.kind == TokenKind::Minus {
                self.advance();

                let expression = self.parse_power()?;

                return Ok(Expr::Negate {
                    expression: Box::new(expression),
                });
            }
        }

        self.parse_primary()
    }
}