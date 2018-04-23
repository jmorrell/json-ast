
mod tokenize;
mod parse;

use tokenize::{tokenize};
use parse::parse_value;
pub use parse::Value;

pub fn parse(input: &str) -> Option<Value> {
    let tokens = tokenize(input);

    let mut index: usize = 0;

    if tokens.len() == 0 {
        println!("this is an error case");
    }

    parse_value(input, &tokens, &mut index)
}
