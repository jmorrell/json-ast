
mod tokenize;
mod parse;
mod types;

use tokenize::{tokenize};
use parse::parse_value;
pub use types::{Node, Parsed};


pub fn parse(input: &str) -> Parsed {
    let tokens = tokenize(input);

    if tokens.len() == 0 {
        Parsed::Failure {
            tree: None,
            errors: vec!(),
        }
    } else {
        parse_value(&tokens)
    }
}
