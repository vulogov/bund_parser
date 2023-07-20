extern crate pest;
use crate::lang::Rule;
use crate::lang::code;
use rust_dynamic::value::Value;
use rust_dynamic::types::*;

pub fn process_token(c: &mut code::Code, _p: &pest::iterators::Pair<Rule>, t: &String) -> bool {
    match Value::from(t.clone()) {
        Ok(str_val) => {
            match str_val.conv(INTEGER) {
                Ok(val) => {
                    c.add_value(val);
                    return false;
                }
                Err(_) => {
                    return true;
                }
            }
        }
        Err(_) => {
            return true;
        }
    }
}
