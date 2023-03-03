use interpreter::{self, tokens::*};

#[test]
fn simple_math_expressions() {
    let minus = String::from("1 - 2");
    let mult = String::from("1 * 2");
    let divide = String::from("1 / 2");
    let res_minus = scan(minus);
    let res_mult = scan(mult);
    let res_divide = scan(divide);

    assert_eq!(res_minus.len(), 3);
    assert_eq!(res_mult.len(), 3);
    assert_eq!(res_divide.len(), 3);
}

#[test]
fn simple_binop() {
    let plus = String::from("1 + 2");
    let res_plus = scan(plus);
    assert_eq!(res_plus.len(), 3);

    let one_enum = res_plus.get(0).unwrap();
    let one = &Token::Number{value: "1".to_string()};
    assert_eq!(one_enum, one);

    assert_eq!(res_plus.get(1).unwrap(), &Token::Add);

    let two = &Token::Number{value: "2".to_string()}; 
    assert_eq!(res_plus.get(2).unwrap(), two);
}

#[test]
/// the string contains some weird spacing just to make sure it works
fn with_parens() {
    let parens = String::from("(1+2  )*   3"); 
    let tokens = scan(parens);
    assert_eq!(tokens.len(), 7);

    assert_eq!(tokens.get(0).unwrap(), &Token::LParen);
    assert_eq!(tokens.get(1).unwrap(), &Token::Number{value: "1".to_string()});
    assert_eq!(tokens.get(2).unwrap(), &Token::Add);
    assert_eq!(tokens.get(3).unwrap(), &Token::Number{value: "2".to_string()});
    assert_eq!(tokens.get(4).unwrap(), &Token::RParen);
    assert_eq!(tokens.get(5).unwrap(), &Token::Mult);
    assert_eq!(tokens.get(6).unwrap(), &Token::Number{value: "3".to_string()});
}

#[test]
fn simple_variable_initialization() {
    let test = String::from("let x = 5;");
    let res = scan(test);

    assert_eq!(res.len(), 5);
    assert!(matches!(res.get(0).unwrap(), Token::Let));

    let _id_id = "x".to_string();
    assert!(matches!(res.get(1).unwrap(), Token::Id{value: _id_id } ));

    assert!(matches!(res.get(2).unwrap(), Token::Assign));

    let _value = "5".to_string();
    assert!(matches!(res.get(3).unwrap(), Token::Number{value: _value}));

    assert!(matches!(res.get(4).unwrap(), Token::Semi));
}

#[test]
fn simple_char_variable() {
    let tokens = scan(String::from("let x='a';"));

    assert_eq!(tokens.len(), 5);
    assert!(matches!(tokens.get(0).unwrap(), Token::Let));
    assert_eq!(tokens.get(1).unwrap(), &Token::Id { value: "x".to_string() });
}

#[test]
fn equality_not_equal() {
    let tokens = scan(String::from("1 == 2"));

    assert_eq!(tokens.len(), 3);
    assert!(matches!(tokens.get(1).unwrap(), Token::Equality));
}
