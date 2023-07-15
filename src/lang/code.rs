
#[derive(Clone)]
pub struct Code {
    count: u64,
}

impl Code {
    pub fn new() -> Self {
        Self {
            count: 0,
        }
    }
}
