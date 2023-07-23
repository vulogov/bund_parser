extern crate pest;
use crate::lang::Rule;
use crate::lang::code;

pub fn process_token(c: &mut code::Code, _p: &pest::iterators::Pair<Rule>, _t: &String) {
    c.new_arg();
}
