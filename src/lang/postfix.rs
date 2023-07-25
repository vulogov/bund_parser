extern crate pest;
use crate::lang::Rule;
use crate::lang::code;

pub fn process_token(c: &mut code::Code, _p: &pest::iterators::Pair<Rule>, t: &String) {
    match c.get_value() {
        Some(mut val) => {
            val.set_tag("postfix", &t);
            c.add_value(val);
        }
        None => {}
    }
}
