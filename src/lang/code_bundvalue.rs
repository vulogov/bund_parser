use rust_dynamic::value::Value;

#[derive(Clone)]
pub struct BundValue {
    pub prefix:     String,
    pub postfix:    String,
    pub value:      Value,
}

impl BundValue {
    pub fn new(v: Value) -> Self {
        Self {
            prefix:     "".to_string(),
            postfix:    "".to_string(),
            value:      v,
        }
    }
}
