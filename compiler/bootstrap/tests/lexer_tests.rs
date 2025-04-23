#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let input = "fn main() { let x = 42; }";
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next_token(), Some(Token::Keyword("fn".into())));
        assert_eq!(lexer.next_token(), Some(Token::Ident("main".into())));
        // ... more test cases ...
    }
}