use std::collections::VecDeque;
use rust_dynamic::value::Value;
use crate::lang::code_bundvalue::BundValue;

#[derive(Clone)]
pub struct Code {
    count: u64,
    prefix: VecDeque<String>,
    vals:  VecDeque<BundValue>,
    args:  VecDeque<VecDeque<BundValue>>
}


impl Code {
    pub fn new() -> Self {
        Self {
            count: 0,
            prefix: VecDeque::new(),
            vals: VecDeque::new(),
            args: VecDeque::new(),
        }
    }
    pub fn get_count(&self) -> u64 {
        self.count
    }
    pub fn inc_count(&mut self) -> u64 {
        self.count += 1;
        self.count
    }
    pub fn add_value(&mut self, v: Value) {
        if self.len_args() > 0 {
            self.add_arg(v);
        } else {
            let mut bv = BundValue::new(v);
            if self.len_prefix() > 0 {
                match self.get_prefix() {
                    Some(prefix) => bv.prefix = prefix,
                    None => {}
                };
            }
            self.vals.push_back(bv);
        }
    }

    pub fn add_bund_value(&mut self, bv: BundValue) {
        if self.len_args() > 0 {
            match self.get_args() {
                Some(mut a) => {
                    a.push_back(bv);
                    self.args.push_back(a);
                }
                None => return,
            }
        } else {
            self.vals.push_back(bv);
        }
    }

    pub fn get_value(&mut self) -> Option<BundValue> {
        if self.len_args() > 0 {
            match self.get_args() {
                Some(mut a) => {
                    let v = a.pop_back();
                    self.args.push_back(a);
                    return v;
                }
                None => None,
            }
        } else {
            self.vals.pop_back()
        }
    }

    pub fn get_actual_value(&mut self) -> Option<BundValue> {
        self.vals.pop_back()
    }
    pub fn add_actual_value(&mut self, bv: BundValue) {
        self.vals.push_back(bv)
    }

    pub fn get_args(&mut self) -> Option<VecDeque<BundValue>> {
        self.args.pop_back()
    }
    pub fn new_arg(&mut self) {
        let a: VecDeque<BundValue> = VecDeque::new();
        self.args.push_back(a);
    }
    pub fn new_arg_value(&mut self, v: Value) {
        let mut a: VecDeque<BundValue> = VecDeque::new();
        let bv = BundValue::new(v);
        a.push_back(bv);
        self.args.push_back(a);
    }
    pub fn add_arg(&mut self, v: Value) {
        match self.get_args() {
            Some(mut a) => {
                let mut bv = BundValue::new(v);
                if self.len_prefix() > 0 {
                    match self.get_prefix() {
                        Some(prefix) => bv.prefix = prefix,
                        None => {}
                    };
                }
                a.push_back(bv);
                self.args.push_back(a);
            }
            None => return,
        }
    }

    pub fn add_prefix(&mut self, p: String) {
        self.prefix.push_back(p)
    }
    pub fn get_prefix(&mut self) -> Option<String> {
        self.prefix.pop_back()
    }

    pub fn len(&mut self) -> usize {
        self.vals.len()
    }
    pub fn len_args(&mut self) -> usize {
        self.args.len()
    }
    pub fn len_prefix(&mut self) -> usize {
        self.prefix.len()
    }
}
