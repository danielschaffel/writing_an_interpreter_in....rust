use std::{iter::Peekable, slice::Chunks};

#[derive(Debug, Clone)]
pub struct NumberToken {
    pub value: String
} 

#[derive(Debug, Clone)]
pub struct RealToken {
    pub value: String
} 

#[derive(Debug, Clone)]
pub struct IdToken {
    pub value: String
} 

#[derive(Debug, Clone)]
pub struct AddToken {} 

#[derive(Debug, Clone)]
pub struct MinusToken {} 

#[derive(Debug, Clone)]
pub struct MultToken {} 

#[derive(Debug, Clone)]
pub struct DivToken {} 

#[derive(Debug, Clone)]
pub struct LParenToken {} 

#[derive(Debug, Clone)]
pub struct RParenToken {} 

#[derive(Debug, Clone)]
pub struct LBraceToken {} 

#[derive(Debug, Clone)]
pub struct RBraceToken {} 

#[derive(Debug, Clone)]
pub struct IfToken {} 

#[derive(Debug, Clone)]
pub struct LetToken {} 

#[derive(Debug, Clone)]
pub struct AssignToken {} 

#[derive(Debug, Clone)]
pub struct EqualityToken {} 

// TODO: add not equal and just negate tokens

#[derive(Debug, Clone)]
pub struct SemiToken {} 

#[derive(Debug, Clone)]
pub enum Token {
    Number(NumberToken),
    Real(RealToken),
    Add(AddToken),
    Minus(MinusToken),
    Mult(MultToken),
    Div(DivToken),
    LParen(LParenToken),
    RParen(RParenToken),
    LBrace(LBraceToken),
    RBrace(RBraceToken),
    IF(IfToken),
    Let(LetToken),
    Assign(AssignToken),
    Equal(EqualityToken),
    Semi(SemiToken),
    Id(IdToken)
}

impl Token {

    pub fn value(&self) -> Option<String> {
        match self {
            Token::Number(c) => Some(Some(c).unwrap().value.to_string()),
            Token::Real(c) => Some(Some(c).unwrap().value.to_string()),
            Token::Add(_) => Some("+".to_string()),
            Token::Minus(_) => Some("-".to_string()),
            Token::Mult(_) => Some("*".to_string()),
            Token::Div(_) => Some("/".to_string()),
            Token::LParen(_) => Some("(".to_string()),
            Token::RParen(_) => Some(")".to_string()),
            Token::LBrace(_) => Some("{".to_string()),
            Token::RBrace(_) => Some("}".to_string()),
            Token::Let(_) => Some("if".to_string()),
            Token::IF(_) => Some("}".to_string()),
            Token::Assign(_) => Some("=".to_string()),
            Token::Equal(_) => Some("==".to_string()),
            Token::Semi(_) => Some(";".to_string()),
            Token::Id(c) => Some(Some(c).unwrap().value.to_string())
        }
    }
}

pub fn scan(input: String) -> Vec<Token> {
    let chars = input.chars().collect::<Vec<char>>();
    let mut tokens: Vec<Token> = Vec::new();
    let mut iter = chars.chunks(1).peekable();
    while iter.peek().is_some() {
        let curr = iter.next();
        println!("{}", curr.unwrap()[0]);
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
                        tokens.push(Token::Number(NumberToken { value: number.clone() }))
                    }
                } else if c[0].is_ascii_alphabetic() {
                    let mut id = String::from(c[0]);

                    if iter.peek().is_some() && valid_id_char(iter.peek().unwrap()[0]) {
                        get_id(&mut iter, &mut id);
                    }
                    if id == "if" {
                        tokens.push(Token::IF(IfToken {}));
                    } else if id == "let" {
                        tokens.push(Token::Let(LetToken {}));
                    } else {
                        tokens.push(Token::Id(IdToken { value: id.clone()}));
                    }


                } else {
                    match c[0] {
                        '=' => {
                            // TODO: figure out == logic
                            tokens.push(Token::Assign(AssignToken {}))
                        },
                        '+' => tokens.push(Token::Add(AddToken {})),
                        '-' => tokens.push(Token::Minus(MinusToken {})),
                        '*' => tokens.push(Token::Mult(MultToken {})),
                        '/' => tokens.push(Token::Div(DivToken {})),
                        '(' => tokens.push(Token::LParen(LParenToken {})),
                        ')' => tokens.push(Token::RParen(RParenToken {})),
                        '{' => tokens.push(Token::LBrace(LBraceToken {})),
                        '}' => tokens.push(Token::RBrace(RBraceToken {})),
                        ';' => tokens.push(Token::Semi(SemiToken {})),
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
                    return Token::Real(RealToken{value: curr.clone()});
                } else if !ch.is_ascii_digit() {
                    break;
                }else {
                    curr.push(ch);
                    if iter.peek().is_some() {
                        match iter.peek().unwrap()[0] {
                            '/' => return Token::Number(NumberToken{ value: curr.clone() }),
                            '*' => return Token::Number(NumberToken{ value: curr.clone() }),
                            '+' => return Token::Number(NumberToken{ value: curr.clone() }),
                            '-' => return Token::Number(NumberToken{ value: curr.clone() }),
                            '(' => return Token::Number(NumberToken{ value: curr.clone() }),
                            ')' => return Token::Number(NumberToken{ value: curr.clone() }),
                            _ => continue
                        }
                    }
                }
            }
        }

    }

    return Token::Number(NumberToken{ value: curr.clone() });
}

fn is_whitespace(c: char) -> bool {
    if c == '\n' || c == '\t' || c == ' ' {
        return true;
    }
    return false;
}
