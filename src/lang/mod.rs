extern crate pest;
use pest::{Parser};
use pest_derive::*;

#[derive(Parser)]
#[grammar = "bund.pest"]
struct BUNDParser;

pub mod parse;
pub mod error;
pub mod code;
pub mod code_bundvalue;
pub mod code_error;
pub mod code_parse;
pub mod code_values;

pub mod token;
pub mod lfb;
pub mod rfb;
pub mod eoi;
pub mod prefix;
pub mod postfix;
pub mod integer;
pub mod float;
pub mod string;
pub mod identifier;
pub mod unknown;

pub fn parse(c: &mut code::Code, s: &String) {
    let pairs = BUNDParser::parse(Rule::program, s);
    match pairs {
        Ok(_) => {
            'outer: while let Ok(ref pair) = pairs {
                for p in pair.clone() {
                    if parse::parse_pair(c, p) {
                        break 'outer;
                    }
                }
            }
        }
        Err(err) => {
            error::parse_error_handler(err);
        }
    }
}
