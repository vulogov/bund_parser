extern crate pest;
use crate::lang::Rule;
use crate::lang::code;

pub fn process_token(_c: &mut code::Code, p: &pest::iterators::Pair<Rule>, t: &String) {
    println!("Received TOKEN token: {:#?}({})", p.as_rule(), t);
}

pub fn post_process_token(_c: &mut code::Code, r: &Rule, t: &String) {
    println!("TOKEN postprocessing: {:?}({})", &r, &t);
}
