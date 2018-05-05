
mod tokenize;
mod parse;
mod types;

use tokenize::{tokenize};
use parse::parse_value;
pub use types::{Node, Parsed};


pub fn parse(input: &str) -> Parsed {
    let tokens = tokenize(input);

    let mut index: usize = 0;

    if tokens.len() == 0 {
        println!("this is an error case");
    }

    parse_value(input, &tokens, &mut index)
}
