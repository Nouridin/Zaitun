use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Keywords
    Let,
    Fn,
    Class,
    If,
    Else,
    While,
    For,
    In,
    Return,
    Try,
    Catch,
    
    // Literals
    Identifier,
    Number,
    String,
    True,
    False,
    
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    EqualEqual,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    
    // Punctuation
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Colon,
    Semicolon,
    Comma,
    Dot,
    Arrow,
    
    // Special
    EOF,
    Error,
}

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} '{}' at {}:{}", 
               self.token_type, self.lexeme, self.line, self.column)
    }
}

pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            column: 1,
        }
    }
    
    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        
        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: String::new(),
            line: self.line,
            column: self.column,
        });
        
        self.tokens.clone()
    }
    
    fn scan_token(&mut self) {
        let c = self.advance();
        
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            ':' => self.add_token(TokenType::Colon),
            ';' => self.add_token(TokenType::Semicolon),
            '+' => self.add_token(TokenType::Plus),
            '-' => {
                if self.match_char('>') {
                    self.add_token(TokenType::Arrow);
                } else {
                    self.add_token(TokenType::Minus);
                }
            },
            '*' => self.add_token(TokenType::Star),
            '/' => {
                if self.match_char('/') {
                    // Comment goes until end of line
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            },
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            },
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::NotEqual);
                } else {
                    self.add_token_error("Unexpected character");
                }
            },
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            },
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            },
            ' ' | '\r' | '\t' => {
                // Ignore whitespace
            },
            '\n' => {
                self.line += 1;
                self.column = 1;
            },
            '"' => self.string(),
            c if self.is_digit(c) => self.number(),
            c if self.is_alpha(c) => self.identifier(),
            _ => self.add_token_error("Unexpected character"),
        }
    }
    
    fn identifier(&mut self) {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }
        
        let text = &self.source[self.start..self.current];
        let token_type = match text {
            "let" => TokenType::Let,
            "fn" => TokenType::Fn,
            "class" => TokenType::Class,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "while" => TokenType::While,
            "for" => TokenType::For,
            "in" => TokenType::In,
            "return" => TokenType::Return,
            "try" => TokenType::Try,
            "catch" => TokenType::Catch,
            "true" => TokenType::True,
            "false" => TokenType::False,
            _ => TokenType::Identifier,
        };
        
        self.add_token(token_type);
    }
    
    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }
        
        // Look for a decimal part
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            // Consume the "."
            self.advance();
            
            while self.is_digit(self.peek()) {
                self.advance();
            }
        }
        
        self.add_token(TokenType::Number);
    }
    
    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 1;
            }
            self.advance();
        }
        
        if self.is_at_end() {
            self.add_token_error("Unterminated string");
            return;
        }
        
        // The closing "
        self.advance();
        
        // Trim the surrounding quotes
        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token(TokenType::String);
    }
    
    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        
        if self.source.chars().nth(self.current) != Some(expected) {
            return false;
        }
        
        self.current += 1;
        self.column += 1;
        true
    }
    
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap_or('\0')
    }
    
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap_or('\0')
    }
    
    fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }
    
    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }
    
    fn is_alphanumeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }
    
    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap_or('\0');
        self.current += 1;
        self.column += 1;
        c
    }
    
    fn add_token(&mut self, token_type: TokenType) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token {
            token_type,
            lexeme: text.to_string(),
            line: self.line,
            column: self.column - (self.current - self.start),
        });
    }
    
    fn add_token_error(&mut self, message: &str) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token {
            token_type: TokenType::Error,
            lexeme: format!("{}: {}", message, text),
            line: self.line,
            column: self.column - (self.current - self.start),
        });
    }
    
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_lexer_simple() {
        let source = "let x = 5;";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.scan_tokens();
        
        assert_eq!(tokens[0].token_type, TokenType::Let);
        assert_eq!(tokens[1].token_type, TokenType::Identifier);
        assert_eq!(tokens[2].token_type, TokenType::Equal);
        assert_eq!(tokens[3].token_type, TokenType::Number);
        assert_eq!(tokens[4].token_type, TokenType::Semicolon);
        assert_eq!(tokens[5].token_type, TokenType::EOF);
    }
    
    #[test]
    fn test_lexer_function() {
        let source = "fn add(a: num, b: num) -> num { return a + b; }";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.scan_tokens();
        
        assert_eq!(tokens[0].token_type, TokenType::Fn);
        assert_eq!(tokens[1].token_type, TokenType::Identifier); // add
        assert_eq!(tokens[2].token_type, TokenType::LeftParen);
        assert_eq!(tokens[3].token_type, TokenType::Identifier); // a
        assert_eq!(tokens[4].token_type, TokenType::Colon);
        assert_eq!(tokens[5].token_type, TokenType::Identifier); // num
        assert_eq!(tokens[6].token_type, TokenType::Comma);
        // ... and so on
    }
}