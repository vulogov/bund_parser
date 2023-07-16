#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use bund_parser::lang::code;
    use bund_parser::lang::parse;

    #[test]
    fn test_string1() {
        let mut c = code::Code::new();
        parse(&mut c, &"\"Hello world\"".to_string());
    }

    #[test]
    fn test_string2() {
        let mut c = code::Code::new();
        parse(&mut c, &"\"This is\nthe place\"[0]".to_string());
    }
}
