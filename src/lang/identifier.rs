extern crate pest;
use crate::lang::Rule;
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

fn process_token_internal(_c: &mut code::Code, p: &pest::iterators::Pair<Rule>, t: &String) -> bool {
    println!("Received identifier token: {:#?} = {}", p.as_rule(), t);
    return false;
}
