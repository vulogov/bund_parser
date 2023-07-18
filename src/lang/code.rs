use std::collections::VecDeque;
use rust_dynamic::value::Value;

#[derive(Clone)]
pub struct Code {
    count: u64,
    vals:  VecDeque<Value>,
}

impl Code {
    pub fn new() -> Self {
        Self {
            count: 0,
            vals: VecDeque::new(),
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
        self.vals.push_back(v);
    }
    pub fn get_value(&mut self) -> Option<Value> {
        self.vals.pop_back()
    }

    pub fn len(&mut self) -> usize {
        self.vals.len()
    }
}
