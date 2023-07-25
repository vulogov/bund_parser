use crate::lang::code;
use std::collections::VecDeque;
use rust_dynamic::value::Value;

fn conv_args(a: VecDeque<Value>) -> Vec<Value> {
    let mut v: Vec<Value> = Vec::new();
    for val in a.iter() {
        v.push(val.clone());
    }
    return v;
}

impl code::Code {
    pub fn is_values(&mut self) -> bool {
        if self.len() == 0 {
            return false;
        }
        true
    }
    pub fn add_arg_to_value(&mut self) {
        if self.len_args() > 0 {
            match self.get_args() {
                Some(args) => {
                    match self.get_value() {
                        Some(mut value) => {
                            value = value.attr(conv_args(args));
                            self.add_value(value);
                        }
                        None => {}
                    }
                }
                None => {}
            }
        }
    }

    pub fn to_values(&mut self) -> Vec<Value> {
        conv_args(self.vals.clone())
    }
}
