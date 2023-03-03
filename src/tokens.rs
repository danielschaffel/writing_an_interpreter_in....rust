use std::{iter::Peekable, slice::Chunks};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number{value: String},
    Real{value: String},
    Char{value: char},
    Add,
    Minus,
    Mult,
    Div,
    LParen,
    RParen,
    LBrace,
    RBrace,
    IF,
    Let,
    Assign,
    Equal,
    Equality,
    Semi,
    Id{ value: String}
}

pub fn scan(input: String) -> Vec<Token> {
    let chars = input.chars().collect::<Vec<char>>();
    let mut tokens: Vec<Token> = Vec::new();
    let mut iter = chars.chunks(1).peekable();
    while iter.peek().is_some() {
        let curr = iter.next();
        match curr {
            Some(c) => {
                if is_whitespace(c[0]) {
                    continue;
                }
                else if c[0].is_ascii_digit() {
                    let mut number = String::from(c[0]);
                    if iter.peek().is_some() && (iter.peek().unwrap()[0].is_ascii_digit() || iter.peek().unwrap()[0] == '.'){
                        let token = get_number(&mut iter, &mut number);
                        tokens.push(token);
                    } else {
                        tokens.push(Token::Number {value: number.clone()})
                    }
                } else if c[0].is_ascii_alphabetic() {
                    let mut id = String::from(c[0]);

                    get_id(&mut iter, &mut id);

                    match id.as_str() {
                        "if" => tokens.push(Token::IF),
                        "let" => tokens.push(Token::Let),
                        _ => tokens.push(Token::Id {value: id.clone()})
                    }
                } else {
                    match c[0] {
                        '\''  => {
                            if iter.peek().unwrap()[0] == '\'' {
                                panic!("Char can't be empty");
                            } else {
                                tokens.push(Token::Char{value: iter.next().unwrap()[0]});

                                if iter.next().unwrap()[0] != '\'' {
                                    panic!("Char must be closed with '");
                                }

                            }
                        },
                        '=' => {
                            // TODO: figure out == logic
                            if iter.peek().unwrap()[0] == '=' {
                                iter.next();
                                tokens.push(Token::Equality);
                            } else {
                                tokens.push(Token::Assign);
                            }
                        },
                        '+' => tokens.push(Token::Add),
                        '-' => tokens.push(Token::Minus),
                        '*' => tokens.push(Token::Mult),
                        '/' => tokens.push(Token::Div),
                        '(' => tokens.push(Token::LParen),
                        ')' => tokens.push(Token::RParen),
                        '{' => tokens.push(Token::LBrace),
                        '}' => tokens.push(Token::RBrace),
                        ';' => tokens.push(Token::Semi),
                        _ => ()
                    }
                }
            }
            None => break
        }
    }

    return tokens;
}

fn get_id(iter: &mut Peekable<Chunks<char>>, id: &mut String) {

    while iter.peek().is_some() && valid_id_char(iter.peek().unwrap()[0]) {
        let curr = iter.next();

        match curr {
            Some(c) => {
                id.push(c[0])
            },
            None => {}
        }
    }
}

fn valid_id_char(c: char) -> bool {
    return c.is_ascii_alphabetic() || c == '_';
}
fn get_number(iter: &mut Peekable<Chunks<char>>, curr: &mut String) -> Token {
    while iter.peek().is_some() {
        let c = iter.next();

        match c {
            None => break,
            Some(c) => {
                let ch = c[0];
                if is_whitespace(ch)  {
                    break;
                } else if ch == '.' {
                    curr.push(ch);
                    get_number(iter, curr);
                    return Token::Real{value: curr.clone()};
                } else if !ch.is_ascii_digit() {
                    break;
                }else {
                    curr.push(ch);
                    if iter.peek().is_some() {
                        match iter.peek().unwrap()[0] {
                            '/' => return Token::Number{ value: curr.clone() },
                            '*' => return Token::Number{ value: curr.clone() },
                            '+' => return Token::Number{ value: curr.clone() },
                            '-' => return Token::Number{ value: curr.clone() },
                            '(' => return Token::Number{ value: curr.clone() },
                            ')' => return Token::Number{ value: curr.clone() },
                            _ => continue
                        }
                    }
                }
            }
        }

    }

    return Token::Number{ value: curr.clone() };
}

fn is_whitespace(c: char) -> bool {
    if c == '\n' || c == '\t' || c == ' ' {
        return true;
    }
    return false;
}
