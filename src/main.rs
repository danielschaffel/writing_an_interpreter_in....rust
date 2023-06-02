use std::io;

#[derive(Debug, PartialEq, Eq)]
enum Token {
    ILLEGAL,
    EOF,
    IDENT(String),
    INT(i64),
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET,
    IF,
    ELSE,
    TRUE,
    FALSE,
    RETURN,
    EQ,
    NOTEQ
}

struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

impl Lexer {
    fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: None,
        };
        l.read_char();
        return l;
    }

    fn read_next_token(&mut self) -> Token {
        let token: Token;
        // println!("read_next_token {:?}", self.ch.unwrap());
        self.skip_whitespace();
        match self.ch {
            None => return Token::EOF,
            Some(c) => match c {
                ';' => token = Token::SEMICOLON,
                '=' => token = {
                    let next = self.peek_next();
                    let t: Token;
                    if next == '=' {
                        self.read_char();
                        t = Token::EQ;
                    } else {
                        t = Token::ASSIGN;
                    }
                    t
                },
                '(' => token = Token::LPAREN,
                ')' => token = Token::RPAREN,
                '-' => token = Token::MINUS,
                '!' => token = {
                    let next = self.peek_next();
                    let t: Token;
                    if next == '=' {
                        self.read_char();
                        t = Token::NOTEQ;
                    } else {
                        t = Token::BANG;
                    }
                    t
                },
                '*' => token = Token::ASTERISK,
                '/' => token = Token::SLASH,
                '<' => token = Token::LT,
                '>' => token = Token::GT,
                ',' => token = Token::COMMA,
                '+' => token = Token::PLUS,
                '{' => token = Token::LBRACE,
                '}' => token = Token::RBRACE,
                _ => {
                    if c.is_ascii_alphabetic() {
                        let val = self.read_identifier();
                        match val.as_str() {
                            "fn" => return Token::FUNCTION,
                            "let" => return Token::LET,
                            "if" => return Token::IF,
                            "else" => return Token::ELSE,
                            "return" => return Token::RETURN,
                            "true" => return Token::TRUE,
                            "false" => return Token::FALSE,
                            _ => token = Token::IDENT(val),
                        }
                        return token;
                    } else if c.is_ascii_digit() {
                        let val = self.read_number();
                        token = Token::INT(val.parse().unwrap());
                        return token; // this seems to fix the weird skipping
                    } else {
                        token = Token::ILLEGAL
                    }
                }
            },
        }

        self.read_char();
        return token;
    }

    fn read_char(&mut self) {
        if self.position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = self.input.chars().nth(self.read_position);
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_next(&self) -> char {
        if self.position >= self.input.len() {
            return '\0';
        } else {
            return self.input.chars().nth(self.read_position).unwrap();
        }
    }

    fn read_identifier(&mut self) -> String {
        let start = self.position;
        while self.ch.unwrap().is_ascii_alphanumeric() {
            self.read_char();
        }
        return String::from(self.input.get(start..self.position).unwrap());
    }
    fn skip_whitespace(&mut self) {
        // println!("start skip whitespace {:?}", self.ch.unwrap());
        while self.ch.unwrap().is_whitespace() {
            self.read_char();
        }
        // println!("end skip whitespace {:?}", self.ch.unwrap());
    }

    fn read_number(&mut self) -> String {
        let start = self.position;
        while self.ch.unwrap().is_ascii_digit() {
            self.read_char();
        }
        // println!("read number end {:?}", self.ch.unwrap());
        return String::from(self.input.get(start..self.position).unwrap());
    }
}

#[cfg(test)]
mod tests {
    use crate::{Lexer, Token};

    #[test]
    fn test_next_token() {
        let input = String::from(
            "let five = 5;
 let ten = 10;
 let add = fn(x,y) {
     x + y;
 };
 
 let result = add(five, ten);
!-/*5;
5 < 10 > 5;
if (5 < 10) {
    return true;
} else {
    return false;
}
10 == 10;
5 != 10;
",
        );
        let mut l = Lexer::new(input);
        let expected = vec![
            Token::LET,
            Token::IDENT(String::from("five")),
            Token::ASSIGN,
            Token::INT(5),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT(String::from("ten")),
            Token::ASSIGN,
            Token::INT(10),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT(String::from("add")),
            Token::ASSIGN,
            Token::FUNCTION,
            Token::LPAREN,
            Token::IDENT(String::from("x")),
            Token::COMMA,
            Token::IDENT(String::from("y")),
            Token::RPAREN,
            Token::LBRACE,
            Token::IDENT(String::from("x")),
            Token::PLUS,
            Token::IDENT(String::from("y")),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT(String::from("result")),
            Token::ASSIGN,
            Token::IDENT(String::from("add")),
            Token::LPAREN,
            Token::IDENT(String::from("five")),
            Token::COMMA,
            Token::IDENT(String::from("ten")),
            Token::RPAREN,
            Token::SEMICOLON,
            Token::BANG,
            Token::MINUS,
            Token::SLASH,
            Token::ASTERISK,
            Token::INT(5),
            Token::SEMICOLON,
            Token::INT(5),
            Token::LT,
            Token::INT(10),
            Token::GT,
            Token::INT(5),
            Token::SEMICOLON,
            Token::IF,
            Token::LPAREN,
            Token::INT(5),
            Token::LT,
            Token::INT(10),
            Token::RPAREN,
            Token::LBRACE,
            Token::RETURN,
            Token::TRUE,
            Token::SEMICOLON,
            Token::RBRACE,
            Token::ELSE,
            Token::LBRACE,
            Token::RETURN,
            Token::FALSE,
            Token::SEMICOLON,
            Token::RBRACE,
            Token::INT(10),
            Token::EQ,
            Token::INT(10),
            Token::SEMICOLON,
            Token::INT(5),
            Token::NOTEQ,
            Token::INT(10),
            Token::SEMICOLON,
        ];

        expected.iter().for_each(|token| {
            let t = l.read_next_token();
            println!("{:?}", t);
            assert_eq!(token, &t);
        })
    }
}

fn main() {
    loop {
        print!(">>");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        input = String::from(input.trim());
        let mut l = Lexer::new(input);
        let mut t = l.read_next_token();
        while t != Token::EOF {
            println!("{:?}", t);
            t = l.read_next_token();
        }
    }
}
