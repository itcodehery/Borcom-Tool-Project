#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Int,
    Return,
    Identifier(String),
    Number(i32),
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,
    EOF,
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn consume(&mut self) -> Option<char> {
        let char = self.peek();
        if char.is_some() {
            self.pos += 1;
        }
        char
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.consume() {
            Some('(') => Token::LeftParen,
            Some(')') => Token::RightParen,
            Some('{') => Token::LeftBrace,
            Some('}') => Token::RightBrace,
            Some(';') => Token::Semicolon,
            Some(c) if c.is_alphabetic() || c == '_' => {
                let mut s = String::new();
                s.push(c);
                while let Some(pc) = self.peek() {
                    if pc.is_alphanumeric() || pc == '_' {
                        s.push(self.consume().unwrap());
                    } else {
                        break;
                    }
                }
                match s.as_str() {
                    "int" => Token::Int,
                    "return" => Token::Return,
                    _ => Token::Identifier(s),
                }
            }
            Some(c) if c.is_digit(10) => {
                let mut s = String::new();
                s.push(c);
                while let Some(pc) = self.peek() {
                    if pc.is_digit(10) {
                        s.push(self.consume().unwrap());
                    } else {
                        break;
                    }
                }
                Token::Number(s.parse().unwrap())
            }
            Some(_) => Token::EOF, // Simplified error handling
            None => Token::EOF,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.consume();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let input = "int main() { return 0; }";
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next_token(), Token::Int);
        assert_eq!(lexer.next_token(), Token::Identifier("main".to_string()));
        assert_eq!(lexer.next_token(), Token::LeftParen);
        assert_eq!(lexer.next_token(), Token::RightParen);
        assert_eq!(lexer.next_token(), Token::LeftBrace);
        assert_eq!(lexer.next_token(), Token::Return);
        assert_eq!(lexer.next_token(), Token::Number(0));
        assert_eq!(lexer.next_token(), Token::Semicolon);
        assert_eq!(lexer.next_token(), Token::RightBrace);
        assert_eq!(lexer.next_token(), Token::EOF);
    }
}
