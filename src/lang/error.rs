use fmterr::fmt_err;
extern crate pest;
use crate::lang::Rule;
use std;
use pest::error::{Error};

pub fn parse_error_handler(err: Error<Rule>) {
    println!("BUND parsing error: {}", fmt_err(&err));
    println!("Error: {:?}", err.variant.message());
    println!("At:    {:?}", err.line());
    // println!("{:?}", err.location.Pos.pos());
}

pub fn exit_at_parse_error_handler(err: Error<Rule>) {
    parse_error_handler(err);
    std::process::exit(0);
}
