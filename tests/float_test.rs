#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use bund_parser::lang::code;

    #[test]
    fn test_float1c() {
        let mut c = code::Code::new();
        c.parse_str("42.0");
        assert_eq!(c.get_count(), 1);
    }

    #[test]
    fn test_float1() {
        let mut c = code::Code::new();
        c.parse_str("42.1");
        assert_eq!(c.len(), 1);
    }

    #[test]
    fn test_float2() {
        let mut c = code::Code::new();
        c.parse_str("3.14[]");
        assert_eq!(c.len(), 1);
    }

    #[test]
    fn test_float3() {
        let mut c = code::Code::new();
        c.parse_str("-42.1");
        assert_eq!(c.len(), 1);
    }
}
