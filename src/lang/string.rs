extern crate pest;
use crate::lang::Rule;
use crate::lang::code;
use rust_dynamic::value::Value;

pub fn process_token(c: &mut code::Code, _p: &pest::iterators::Pair<Rule>, t: &String) -> bool {
    let mut value = Value::from_string(t.clone());
    if c.len_prefix() > 0 {
        match c.get_prefix() {
            Some(prefix) => {
                value.set_tag("prefix", &prefix);
            }
            None => return true,
        }
    }
    c.add_value(value);
    return false;
}
