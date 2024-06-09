#[derive(Clone, Debug)]
pub enum SqlValue {
    I128(i128),
    I64(i64),
    I32(i32),
    U128(u128),
    U64(u64),
    U32(u32),
    F64(f64),
    F32(f32),
    Bool(bool),
    Text(String),
    Int(i32),
    Float(f32),
    Bytes(Vec<u8>),
    StringDate(String), // For date values
    Null,
}

impl SqlValue {
    pub fn to_sql(&self) -> String {
        match self {
            Self::Null => "NULL".to_string(),
            Self::I128(val) => val.parse_sql_value(),
            Self::I64(val) => val.parse_sql_value(),
            Self::I32(val) => val.parse_sql_value(),
            Self::U128(val) => val.parse_sql_value(),
            Self::U64(val) => val.parse_sql_value(),
            Self::U32(val) => val.parse_sql_value(),
            Self::F64(val) => val.parse_sql_value(),
            Self::F32(val) => val.parse_sql_value(),
            Self::Bool(val) => val.parse_sql_value(),
            Self::Text(val) => val.parse_sql_value(),
            Self::Int(val) => val.parse_sql_value(),
            Self::Float(val) => val.parse_sql_value(),
            Self::StringDate(val) => val.parse_sql_value(),
            _ => "".to_owned(),
        }
    }
    pub fn from_string_slice(val: &str) -> Self {
        Self::Text(val.to_string())
    }
}
trait SqlValueParser {
    fn parse_sql_value(&self) -> String;
}

macro_rules! impl_sql_parser {
    ($($t: ty),+) => {

            $(
                impl SqlValueParser for $t {
                        fn parse_sql_value(&self) -> String {
                            format!("{}", self)
                        }
                }
            )+

    };

 (true, $($t: ty),+) => {
    $(
        impl SqlValueParser for $t {
            fn parse_sql_value(&self) -> String {
                let mut str = String::new();
                for c in self.to_string().chars() {
                    if c == '\'' {
                        str.push_str("\'\'")
                    } else {
                        str.push(c);
                    }
                }
                format!("\'{}\'", str)
            }
        }
    )+
};}

impl_sql_parser!(true, String, &str, char);

impl_sql_parser!(u8, u32, u16, u64, u128, i8, i16, i32, i64, i128, f32, f64, bool);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sql_value_null() {
        let val = SqlValue::Null;
        assert_eq!(val.to_sql(), "NULL");
    }

    #[test]
    fn test_sql_value_i128() {
        let val = SqlValue::I128(123456789012345678901234567890);
        assert_eq!(val.to_sql(), "123456789012345678901234567890");
    }

    #[test]
    fn test_sql_value_i64() {
        let val = SqlValue::I64(1234567890123456789);
        assert_eq!(val.to_sql(), "1234567890123456789");
    }

    #[test]
    fn test_sql_value_i32() {
        let val = SqlValue::I32(1234567890);
        assert_eq!(val.to_sql(), "1234567890");
    }

    #[test]
    fn test_sql_value_u128() {
        let val = SqlValue::U128(123456789012345678901234567890);
        assert_eq!(val.to_sql(), "123456789012345678901234567890");
    }

    #[test]
    fn test_sql_value_u64() {
        let val = SqlValue::U64(1234567890123456789);
        assert_eq!(val.to_sql(), "1234567890123456789");
    }

    #[test]
    fn test_sql_value_u32() {
        let val = SqlValue::U32(1234567890);
        assert_eq!(val.to_sql(), "1234567890");
    }

    #[test]
    fn test_sql_value_f64() {
        let val = SqlValue::F64(1234.5678);
        assert_eq!(val.to_sql(), "1234.5678");
    }

    #[test]
    fn test_sql_value_f32() {
        let val = SqlValue::F32(1234.5679);
        assert_eq!(val.to_sql(), "1234.5679"); // Floating-point precision issue
    }

    #[test]
    fn test_sql_value_bool() {
        let val = SqlValue::Bool(true);
        assert_eq!(val.to_sql(), "true");
    }

    #[test]
    fn test_sql_value_text() {
        let val = SqlValue::Text("Hello, World!".to_string());
        assert_eq!(val.to_sql(), "'Hello, World!'");
    }

    #[test]
    fn test_sql_value_int() {
        let val = SqlValue::Int(-123);
        assert_eq!(val.to_sql(), "-123");
    }

    #[test]
    fn test_sql_value_float() {
        let val = SqlValue::Float(-123.45);
        assert_eq!(val.to_sql(), "-123.45");
    }

    #[test]
    fn test_sql_value_string_date() {
        let val = SqlValue::StringDate("2024-06-12".to_string());
        assert_eq!(val.to_sql(), "'2024-06-12'");
    }
    #[test]
    fn test_sql_value_char() {
        let val = SqlValue::from_string_slice("A");
        assert_eq!(val.to_sql(), "'A'");
    }

    #[test]
    fn test_sql_value_str() {
        let val = SqlValue::from_string_slice("Hello");
        assert_eq!(val.to_sql(), "'Hello'");
    }

    #[test]
    fn test_sql_value_string() {
        let val = SqlValue::from_string_slice(&String::from("World"));
        assert_eq!(val.to_sql(), "'World'");
    }

    #[test]
    fn test_sql_value_true() {
        let val = SqlValue::Bool(true);
        assert_eq!(val.to_sql(), "true");
    }

    #[test]
    fn test_sql_value_false() {
        let val = SqlValue::Bool(false);
        assert_eq!(val.to_sql(), "false");
    }

    #[test]
    fn test_sql_value_uchar() {
        let val = SqlValue::U32(255);
        assert_eq!(val.to_sql(), "255");
    }

    #[test]
    fn test_sql_value_usize() {
        let val = SqlValue::U32(123456);
        assert_eq!(val.to_sql(), "123456");
    }

    #[test]
    fn test_sql_value_str_special() {
        let val = SqlValue::from_string_slice("Let's go");
        assert_eq!(val.to_sql(), "'Let''s go'");
    }

    #[test]
    fn test_sql_value_string_special() {
        let val = SqlValue::from_string_slice(&String::from("It's raining"));
        assert_eq!(val.to_sql(), "'It''s raining'");
    }
}
