#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use bund_parser::lang::code;

    #[test]
    fn test_parse_int_1() {
        let mut c = code::Code::new();
        c.parse_str("42");
        let values = c.to_values();
        assert_eq!(values[0].cast_int().unwrap(), 42);
    }

    #[test]
    fn test_parse_int_2() {
        let mut c = code::Code::new();
        c.parse_str("42[0]");
        let values = c.to_values();
        assert_eq!(values.len(), 1);
    }

    #[test]
    fn test_parse_int_3() {
        let mut c = code::Code::new();
        c.parse_str("42[0]");
        let values = c.to_values();
        assert_eq!(values[0].attr[0].cast_int().unwrap(), 0);
    }

    #[test]
    fn test_parse_int_4() {
        let mut c = code::Code::new();
        c.parse_str("*42[0]");
        let mut values = c.to_values();
        assert_eq!(values[0].get_tag("prefix").unwrap(), "*");
    }

    #[test]
    fn test_parse_int_5() {
        let mut c = code::Code::new();
        c.parse_str("*42?[0]");
        let mut values = c.to_values();
        assert_eq!(values[0].get_tag("postfix").unwrap(), "?");
    }

    #[test]
    fn test_parse_float_1() {
        let mut c = code::Code::new();
        c.parse_str("3.14");
        let values = c.to_values();
        assert_eq!(values[0].cast_float().unwrap(), 3.14);
    }

    #[test]
    fn test_parse_float_2() {
        let mut c = code::Code::new();
        c.parse_str("3.14[0]");
        let values = c.to_values();
        assert_eq!(values.len(), 1);
    }

    #[test]
    fn test_parse_float_3() {
        let mut c = code::Code::new();
        c.parse_str("3.14[0]");
        let values = c.to_values();
        assert_eq!(values[0].attr[0].cast_int().unwrap(), 0);
    }

    #[test]
    fn test_parse_float_4() {
        let mut c = code::Code::new();
        c.parse_str("*3.14[0]");
        let mut values = c.to_values();
        assert_eq!(values[0].get_tag("prefix").unwrap(), "*");
    }

    #[test]
    fn test_parse_float_5() {
        let mut c = code::Code::new();
        c.parse_str("*3.14?[0]");
        let mut values = c.to_values();
        assert_eq!(values[0].get_tag("postfix").unwrap(), "?");
    }

    #[test]
    fn test_parse_string_1() {
        let mut c = code::Code::new();
        c.parse_str("'Hello'");
        let values = c.to_values();
        assert_eq!(values[0].cast_string().unwrap(), "Hello");
    }

    #[test]
    fn test_parse_string_1_1() {
        let mut c = code::Code::new();
        c.parse_str("\"Hello\"");
        let values = c.to_values();
        assert_eq!(values[0].cast_string().unwrap(), "Hello");
    }

    #[test]
    fn test_parse_string_2() {
        let mut c = code::Code::new();
        c.parse_str("'Hello'['World']");
        let values = c.to_values();
        assert_eq!(values.len(), 1);
    }

    #[test]
    fn test_parse_string_3() {
        let mut c = code::Code::new();
        c.parse_str("'Hello'['World']");
        let values = c.to_values();
        assert_eq!(values[0].attr[0].cast_string().unwrap(), "World");
    }

    #[test]
    fn test_parse_string_4() {
        let mut c = code::Code::new();
        c.parse_str("*'Hello'['World']");
        let mut values = c.to_values();
        assert_eq!(values[0].get_tag("prefix").unwrap(), "*");
    }

    #[test]
    fn test_parse_string_5() {
        let mut c = code::Code::new();
        c.parse_str("*'Hello'?['World']");
        let mut values = c.to_values();
        assert_eq!(values[0].get_tag("postfix").unwrap(), "?");
    }
}
