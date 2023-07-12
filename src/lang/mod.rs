extern crate pest;
use pest::{Parser};
use pest_derive::*;

#[derive(Parser)]
#[grammar = "bund.pest"]
struct BUNDParser;

pub mod parse;
pub mod error;

pub fn parse(s: &String) {
    let pairs = BUNDParser::parse(Rule::program, s);
    match pairs {
        Ok(_) => {
            while let Ok(ref pair) = pairs {
                for p in pair.clone() {
                    parse::parse_pair(p);
                }
            }
        }
        Err(err) => {
            error::parse_error_handler(err);
        }
    }
}
