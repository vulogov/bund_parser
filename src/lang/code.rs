use std::collections::VecDeque;
use rust_dynamic::value::Value;

#[derive(Clone)]
pub struct Code {
    count:      u64,
    prefix:     VecDeque<String>,
    pub vals:   VecDeque<Value>,
    args:       VecDeque<VecDeque<Value>>
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
    pub fn add_value(&mut self, mut v: Value) {
        if self.len_prefix() > 0 {
            match self.get_prefix() {
                Some(prefix) => v.set_tag("prefix", &prefix),
                None => {}
            };
        }
        if self.len_args() > 0 {
            self.add_arg(v);
        } else {
            self.vals.push_back(v);
        }
    }

    pub fn get_value(&mut self) -> Option<Value> {
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

    pub fn get_args(&mut self) -> Option<VecDeque<Value>> {
        self.args.pop_back()
    }
    pub fn new_arg(&mut self) {
        let a: VecDeque<Value> = VecDeque::new();
        self.args.push_back(a);
    }
    pub fn new_arg_value(&mut self, v: Value) {
        let mut a: VecDeque<Value> = VecDeque::new();
        a.push_back(v);
        self.args.push_back(a);
    }
    pub fn add_arg(&mut self, mut v: Value) {
        match self.get_args() {
            Some(mut a) => {
                if self.len_prefix() > 0 {
                    match self.get_prefix() {
                        Some(prefix) => v.set_tag("prefix", &prefix),
                        None => {}
                    };
                }
                a.push_back(v);
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
