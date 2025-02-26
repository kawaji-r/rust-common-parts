//! &str -> i32
//! &str -> String
//! i32 -> String
//! i32 -> u8
//! String -> &str
//! String -> i32

#[cfg(test)]
mod tests {
    // as
    #[test]
    fn test_as_conversion() {
        let x: i32 = 10;
        let y: f64 = x as f64; // i32 から f64 へ変換
        assert_eq!(y, 10.0);
    }

    // &str -> i32
    #[test]
    fn test_parse_conversion() {
        let s = "42";
        // &str を i32 に変換。変換失敗時は panic するため expect() を利用
        let num: i32 = s.parse().expect("Failed to parse string to i32");
        assert_eq!(num, 42);
    }

    // &str -> String
    #[test]
    fn test_from_into_conversion() {
        let s: &str = "hello";
        // From トレイトを使った変換
        let string_from = String::from(s);
        // Into トレイトを使った変換（暗黙的に From を呼び出す）
        let string_into: String = s.into();
        assert_eq!(string_from, "hello");
        assert_eq!(string_into, "hello");
    }

    // i32 -> String
    #[test]
    fn test_to_string_conversion() {
        let num = 123;
        let s = num.to_string();
        assert_eq!(s, "123");
    }

    // i32 -> u8
    #[test]
    fn test_try_from_conversion() {
        use std::convert::TryFrom;
        let i: i32 = 10;
        // i32 -> u8 への変換。10 は u8 に収まるため成功する
        let u: u8 = u8::try_from(i).expect("Conversion failed");
        assert_eq!(u, 10u8);
    }

    // String -> &str
    #[test]
    fn test_string_to_str() {
        let s: String = String::from("Hello, Rust!");
        // 方法1: as_str() を利用
        let s_str: &str = s.as_str();
        assert_eq!(s_str, "Hello, Rust!");

        // 方法2: Deref トレイトを利用
        let s_str2: &str = &s;
        assert_eq!(s_str2, "Hello, Rust!");
    }

    // String -> i32
    #[test]
    fn test_string_to_i32() {
        let s: String = String::from("42");
        // parse() は Result 型を返すため、エラー処理が必要です
        let num: i32 = s.parse().expect("有効な i32 に変換できませんでした");
        assert_eq!(num, 42);
    }
}
