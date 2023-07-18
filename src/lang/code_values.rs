use crate::lang::code;

impl code::Code {
    pub fn is_values(&mut self) -> bool {
        if self.len() == 0 {
            return false;
        }
        true
    }
}
