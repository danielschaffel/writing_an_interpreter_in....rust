pub mod tokens;

use crate::tokens::scan;

fn main() {
    let test = String::from("let x = 5;");
    scan(test).iter().for_each(|t| println!("{:?}", t));

}

