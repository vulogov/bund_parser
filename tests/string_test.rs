#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use bund_parser::lang::code;

    #[test]
    fn test_string1c() {
        let mut c = code::Code::new();
        c.parse_str("\"\"");
        assert_eq!(c.get_count(), 1);
    }

    #[test]
    fn test_string1() {
        let mut c = code::Code::new();
        c.parse_str("\"Hello world\"");
        assert_eq!(c.len(), 1);
    }

    #[test]
    fn test_string2() {
        let mut c = code::Code::new();
        c.parse_str("\"This is\nthe place\"[0]");
        assert_eq!(c.len(), 1);
    }

    #[test]
    fn test_string3() {
        let mut c = code::Code::new();
        c.parse_str("'Literals are strings too'");
        assert_eq!(c.len(), 1);
    }
}
