
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Number(String),

    Plus,       // ~
    Minus,      // |||
    Power,      // ##

    SquareRoot, // V_

    LeftParen,
    RightParen,

    LeftBrace,
    RightBrace,

    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub position: usize,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn current(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    fn peek(&self, offset: usize) -> Option<char> {
        self.input.get(self.position + offset).copied()
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.current(), Some(c) if c.is_whitespace()) {
            self.advance();
        }
    }

    fn read_number(&mut self) -> Result<TokenKind, String> {
        let start = self.position;

        // DBM-число в вертикальных палках.
        if self.current() == Some('|') {
            self.advance();

            while let Some(c) = self.current() {
                if c == '|' {
                    self.advance();

                    let raw: String = self.input[start..self.position]
                        .iter()
                        .collect();

                    return Ok(TokenKind::Number(raw));
                }

                self.advance();
            }

            return Err(format!(
                "Unclosed DBM number at position {start}"
            ));
        }

        // Отрицательное DBM-число.
        if self.current() == Some('-') {
            self.advance();

            while matches!(self.current(), Some(c) if c.is_ascii_digit()) {
                self.advance();
            }

            if self.current() == Some('+') {
                self.advance();

                while matches!(self.current(), Some(c) if c.is_ascii_digit()) {
                    self.advance();
                }
            }

            let raw: String = self.input[start..self.position]
                .iter()
                .collect();

            return Ok(TokenKind::Number(raw));
        }

        // Обычное число.
        while matches!(self.current(), Some(c) if c.is_ascii_digit() || c == '.') {
            self.advance();
        }

        let raw: String = self.input[start..self.position]
            .iter()
            .collect();

        Ok(TokenKind::Number(raw))
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        loop {
            self.skip_whitespace();

            let position = self.position;

            let kind = match self.current() {
                None => {
                    tokens.push(Token {
                        kind: TokenKind::EOF,
                        position,
                    });

                    break;
                }

                Some('~') => {
                    self.advance();
                    TokenKind::Plus
                }

                Some('|') => {
                    if self.peek(1) == Some('|') && self.peek(2) == Some('|') {
                        self.position += 3;
                        TokenKind::Minus
                    } else {
                        self.read_number()?
                    }
                }

                Some('#') => {
                    if self.peek(1) == Some('#') {
                        self.position += 2;
                        TokenKind::Power
                    } else {
                        return Err(format!(
                            "Unexpected '#' at position {position}"
                        ));
                    }
                }

                Some('V') => {
                    if self.peek(1) == Some('_') {
                        self.position += 2;
                        TokenKind::SquareRoot
                    } else {
                        return Err(format!(
                            "Unexpected 'V' at position {position}"
                        ));
                    }
                }

                Some('(') => {
                    self.advance();
                    TokenKind::LeftParen
                }

                Some(')') => {
                    self.advance();
                    TokenKind::RightParen
                }

                Some('{') => {
                    self.advance();
                    TokenKind::LeftBrace
                }

                Some('}') => {
                    self.advance();
                    TokenKind::RightBrace
                }

                Some('-') if self.is_negative_dbm_number() => {
                    self.read_number()?
                }

                Some('-') => {
                    self.advance();
                    TokenKind::Minus
                }

                Some(c) if c.is_ascii_digit() => {
                    self.read_number()?
                }

                Some(c) => {
                    return Err(format!(
                        "Unexpected character '{c}' at position {position}"
                    ));
                }
            };

            tokens.push(Token {
                kind,
                position,
            });
        }

        Ok(tokens)
    }

    fn is_negative_dbm_number(&self) -> bool {
        let mut offset = 1;

        // Пропускаем цифры после минуса.
        while matches!(self.peek(offset), Some(c) if c.is_ascii_digit()) {
            offset += 1;
        }

        // После цифр должен идти плюс.
        if self.peek(offset) != Some('+') {
            return false;
        }

        offset += 1;

        // После плюса должна быть хотя бы одна цифра.
        matches!(self.peek(offset), Some(c) if c.is_ascii_digit())
    }
}
