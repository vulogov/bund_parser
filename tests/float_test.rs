#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use bund_parser::lang::code;
    use bund_parser::lang::parse;

    #[test]
    fn test_float1() {
        let mut c = code::Code::new();
        parse(&mut c, &"42.1".to_string());
    }

    #[test]
    fn test_float2() {
        let mut c = code::Code::new();
        parse(&mut c, &"3.14[0]".to_string());
    }

    #[test]
    fn test_float3() {
        let mut c = code::Code::new();
        parse(&mut c, &"-42.1".to_string());
    }
}
