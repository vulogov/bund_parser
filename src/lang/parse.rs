extern crate pest;
use crate::lang::Rule;
use crate::lang::code;

use crate::lang::token;
use crate::lang::lfb;
use crate::lang::rfb;
use crate::lang::prefix;
use crate::lang::postfix;
use crate::lang::integer;
use crate::lang::float;
use crate::lang::string;
use crate::lang::identifier;
use crate::lang::eoi;
use crate::lang::unknown;


pub fn parse_pair(c: &mut code::Code, p: pest::iterators::Pair<Rule>) -> bool {
    let rule  = &p.as_rule();
    let token = &p.as_span();

    match rule {
        Rule::term => {
            token::process_token(c, &p, &token.as_str().to_string());
            for inner in p.into_inner() {
                parse_pair(c, inner);
            }
            token::post_process_token(c, &rule, &token.as_str().to_string());
        }
        Rule::prefix => {
            prefix::process_token(c, &p, &token.as_str().to_string());
        }
        Rule::postfix => {
            prefix::process_token(c, &p, &token.as_str().to_string());
        }
        Rule::integer => {
            integer::process_token(c, &p, &token.as_str().to_string());
        }
        Rule::float => {
            float::process_token(c, &p, &token.as_str().to_string());
        }
        Rule::string => {
            string::process_token(c, &p, &token.as_str().to_string());
        }
        Rule::multi_line_string => {
            string::process_token(c, &p, &token.as_str().to_string());
        }
        Rule::literal => {
            string::process_token(c, &p, &token.as_str().to_string());
        }
        Rule::letter_ident => {
            return identifier::process_token(c, &p, &token.as_str().to_string());
        }
        Rule::op_ident => {
            return identifier::process_op_token(c, &p, &token.as_str().to_string());
        }
        Rule::left_function_bracket => {
            lfb::process_token(c, &p, &token.as_str().to_string());
        }
        Rule::right_function_bracket => {
            rfb::process_token(c, &p, &token.as_str().to_string());
        }
        Rule::EOI => {
            eoi::process_token(c, &p, &token.as_str().to_string());
            return true;
        }
        _ => {
            unknown::process_token(c, &p, &token.as_str().to_string());
            return true;
        }
    }
    return false;
}
