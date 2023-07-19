extern crate pest;
use crate::lang::Rule;
use crate::lang::code;
use rust_dynamic::value::Value;

pub fn process_token(c: &mut code::Code, _p: &pest::iterators::Pair<Rule>, t: &String) {
    c.add_value(Value::from_string(t));
}
