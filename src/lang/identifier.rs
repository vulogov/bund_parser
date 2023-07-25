extern crate pest;
use crate::lang::Rule;
use rust_dynamic::value::Value;
use crate::lang::code;

pub fn process_token(c: &mut code::Code, p: &pest::iterators::Pair<Rule>, t: &String) -> bool {
    return process_token_internal(c, p, t);
}

pub fn process_op_token(c: &mut code::Code, p: &pest::iterators::Pair<Rule>, t: &String) -> bool {
    if t.len() <= 2 {
        println!("OP Identifier is too small");
        return true;
    }
    let mut op_t = t.clone();
    op_t.pop();
    if op_t.len() > 0 {
        op_t.remove(0);
    }
    return process_token_internal(c, p, &op_t);
}

fn process_token_internal(c: &mut code::Code, p: &pest::iterators::Pair<Rule>, t: &String) -> bool {
    let _ = &c.inc_count();
    if c.len_prefix() > 0 {
        match c.get_prefix() {
            Some(prefix) => {
                create_call_value_with_prefix(c, p, t, prefix);
            }
            None => {
                create_call_value_no_prefix(c, p, t);
            }
        }
    }
    return false;
}

fn create_call_value_with_prefix(c: &mut code::Code, _p: &pest::iterators::Pair<Rule>, t: &String, pfx: String) {
    match pfx.as_str() {
        "`" => {
            let mut val = Value::ptr(t.to_string(), Vec::<Value>::new());
            val.set_tag("prefix", &pfx);
            c.add_value(val);
        }
        _ => {
            let mut val = Value::call(t.to_string(), Vec::<Value>::new());
            val.set_tag("prefix", &pfx);
            c.add_value(val);
        }
    }
}

fn create_call_value_no_prefix(c: &mut code::Code, _p: &pest::iterators::Pair<Rule>, t: &String) {
    match t.as_str() {
        "lambda" => {
            let val =  Value::lambda();
            c.add_value(val)
        }
        "ℷ" => {
            let val =  Value::lambda();
            c.add_value(val)
        }
        _ => {
            let val =  Value::call(t.to_string(), Vec::<Value>::new());
            c.add_value(val);
        }
    }
}
