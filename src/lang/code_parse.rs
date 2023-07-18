use crate::lang::code;
use crate::lang::parse;

impl code::Code {
    pub fn parse(&mut self, code: &String) {
        parse(self, code);
    }
    pub fn parse_str(&mut self, code: &str) {
        self.parse(&code.to_string());
    }
}
