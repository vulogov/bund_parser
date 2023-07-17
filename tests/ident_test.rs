#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use bund_parser::lang::code;
    use bund_parser::lang::parse;

    #[test]
    fn test_ident1() {
        let mut c = code::Code::new();
        parse(&mut c, &"abc".to_string());
    }

    #[test]
    fn test_ident2() {
        let mut c = code::Code::new();
        parse(&mut c, &"abc.cde".to_string());
    }

    #[test]
    fn test_ident3() {
        let mut c = code::Code::new();
        parse(&mut c, &"(+)".to_string());
    }

    #[test]
    fn test_ident4() {
        let mut c = code::Code::new();
        parse(&mut c, &"(===)".to_string());
    }

    #[test]
    fn test_ident5() {
        let mut c = code::Code::new();
        parse(&mut c, &"(lambda+)".to_string());
    }
}
