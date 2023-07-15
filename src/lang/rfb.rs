extern crate pest;
use crate::lang::Rule;
use crate::lang::code;

pub fn process_token(_c: &mut code::Code, p: &pest::iterators::Pair<Rule>, _t: &String) {
    println!("Received RFB token: {:#?}", p.as_rule());
}
