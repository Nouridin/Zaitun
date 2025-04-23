use crate::lexer::{Token, Lexer};
use crate::ast::*;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Parser { lexer, current_token: None };
        parser.advance();
        parser
    }

    pub fn parse(&mut self) -> AST {
        let mut nodes = vec![];
        while self.current_token.is_some() {
            nodes.push(self.parse_statement());
        }
        AST::new(nodes)
    }

    fn parse_statement(&mut self) -> ASTNode {
        match self.current_token {
            Some(Token::Keyword(ref kw)) if kw == "fn" => self.parse_function(),
            Some(Token::Keyword(ref kw)) if kw == "struct" => self.parse_struct(),
            _ => self.parse_expression(),
        }
    }
    
    fn parse_loop(&mut self) -> ASTNode {
        self.expect(Token::Keyword("loop".into()));
        let body = self.parse_block();
        ASTNode::Loop { body }
    }

    fn parse_for(&mut self) -> ASTNode {
        self.expect(Token::Keyword("for".into()));
        let init = self.parse_variable_decl();
        self.expect(Token::Symbol(";".into()));
        let condition = self.parse_expression();
        self.expect(Token::Symbol(";".into()));
        let update = self.parse_expression();
        let body = self.parse_block();
        ASTNode::For {
            init,
            condition,
            update,
            body,
        }
    }
}