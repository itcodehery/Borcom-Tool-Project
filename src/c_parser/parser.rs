use crate::c_parser::lexer::{Lexer, Token};
use crate::c_parser::ast::*;

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        Parser { lexer, current_token }
    }

    fn peek(&self) -> &Token {
        &self.current_token
    }

    fn consume(&mut self, expected: Token) {
        if self.current_token == expected {
            self.current_token = self.lexer.next_token();
        } else {
            panic!("Expected {:?}, found {:?}", expected, self.current_token);
        }
    }

    pub fn parse_program(&mut self) -> Program {
        Program::Function(self.parse_function())
    }

    fn parse_function(&mut self) -> Function {
        self.consume(Token::Int);
        let name = if let Token::Identifier(name) = self.peek().clone() {
            self.consume(Token::Identifier(name.clone()));
            name
        } else {
            panic!("Expected identifier, found {:?}", self.peek());
        };

        self.consume(Token::LeftParen);
        self.consume(Token::RightParen);
        self.consume(Token::LeftBrace);
        
        let mut body = Vec::new();
        while self.peek() != &Token::RightBrace {
            body.push(self.parse_statement());
        }
        
        self.consume(Token::RightBrace);

        Function { name, body }
    }

    fn parse_statement(&mut self) -> Statement {
        match self.peek().clone() {
            Token::Return => {
                self.consume(Token::Return);
                let expr = self.parse_expr();
                self.consume(Token::Semicolon);
                Statement::Return(expr)
            }
            t => panic!("Unexpected token in statement: {:?}", t),
        }
    }

    fn parse_expr(&mut self) -> Expr {
        match self.peek().clone() {
            Token::Number(val) => {
                self.consume(Token::Number(val));
                Expr::Number(val)
            }
            t => panic!("Unexpected token in expression: {:?}", t),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::c_parser::lexer::Lexer;

    #[test]
    fn test_parser() {
        let input = "int main() { return 2; }";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        let expected = Program::Function(Function {
            name: "main".to_string(),
            body: vec![Statement::Return(Expr::Number(2))],
        });

        assert_eq!(program, expected);
    }
}
