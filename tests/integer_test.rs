#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use bund_parser::lang::code;

    #[test]
    fn test_int1c() {
        let mut c = code::Code::new();
        c.parse_str("42");
        assert_eq!(c.get_count(), 1);
    }

    #[test]
    fn test_int1() {
        let mut c = code::Code::new();
        c.parse_str("42");
        assert_eq!(c.len(), 1);
    }

    #[test]
    fn test_int2() {
        let mut c = code::Code::new();
        c.parse_str("42[0]");
    }

    #[test]
    fn test_int3() {
        let mut c = code::Code::new();
        c.parse_str("+42");
        assert_eq!(c.len(), 1);
    }
}
