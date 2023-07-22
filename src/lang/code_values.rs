use crate::lang::code;
use std::collections::VecDeque;
use rust_dynamic::value::Value;
use crate::lang::code_bundvalue::BundValue;

fn conv_args(a: VecDeque<BundValue>) -> Vec<Value> {
    let mut v: Vec<Value> = vec![];
    return v;
}

impl code::Code {
    pub fn is_values(&mut self) -> bool {
        if self.len() == 0 {
            return false;
        }
        true
    }

    pub fn assign_args(&mut self) -> bool {
        match self.get_args() {
            Some(args) => {
                match self.get_value() {
                    Some(mut bv) => {
                        bv.value.attr(conv_args(args));
                        self.add_bund_value(bv);
                        return true;
                    }
                    None => return false,
                }
            }
            None => return false,
        }
    }
}
