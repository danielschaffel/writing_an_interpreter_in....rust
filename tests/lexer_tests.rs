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
fn simple_plus() {
    let plus = String::from("1 + 2");
    let res_plus = scan(plus);
    assert_eq!(res_plus.len(), 3);

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
