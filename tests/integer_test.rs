#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use bund_parser::lang::code;
    use bund_parser::lang::parse;

    #[test]
    fn test_int1() {
        let mut c = code::Code::new();
        parse(&mut c, &"42".to_string());
    }

    #[test]
    fn test_int2() {
        let mut c = code::Code::new();
        parse(&mut c, &"42[0]".to_string());
    }

    #[test]
    fn test_int3() {
        let mut c = code::Code::new();
        parse(&mut c, &"+42".to_string());
    }
}
