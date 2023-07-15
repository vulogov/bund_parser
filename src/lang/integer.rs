extern crate pest;
use crate::lang::Rule;
use crate::lang::code;

pub fn process_token(_c: &mut code::Code, p: &pest::iterators::Pair<Rule>, t: &String) {
    println!("Received integer token: {:#?} = {}", p.as_rule(), t);
}
