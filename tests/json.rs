#![cfg(feature = "use_serde")]

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_json() {
        let json_str = r#"
            {
                "name": "Alice",
                "age": 30,
                "phones": ["123-4567", "234-5678"]
            }
            "#;

        // JSON文字列を serde_json::Value にパース
        let v: Value = serde_json::from_str(json_str).unwrap();

        // 各フィールドにアクセスする
        assert_eq!(v["name"], "Alice");
        assert_eq!(v["age"], 30);

        let phones = v["phones"].as_array().unwrap();
        assert_eq!(phones.len(), 2);
        assert_eq!(phones[0], "123-4567");
    }

    #[test]
    fn test_create_json() {
        // json! マクロで JSON を生成
        let v = json!({
            "name": "Bob",
            "age": 25,
            "phones": ["345-6789"]
        });

        // 整形済みのJSON文字列にシリアライズ
        let pretty = serde_json::to_string_pretty(&v).unwrap();
        println!("生成したJSON:\n{}", pretty);

        // 文字列から再度パース
        let v2: Value = serde_json::from_str(&pretty).unwrap();
        assert_eq!(v, v2);
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Person {
        name: String,
        age: u32,
        phones: Vec<String>,
    }

    #[test]
    fn test_json_to_struct() {
        let json_str = r#"
            {
                "name": "Alice",
                "age": 30,
                "phones": ["123-4567", "234-5678"]
            }
            "#;

        // JSON文字列から Person にデシリアライズ
        let person: Person = serde_json::from_str(&json_str).unwrap();
        assert_eq!(person.name, "Alice");
        assert_eq!(person.age, 30);

        assert_eq!(person.phones.len(), 2);
        assert_eq!(person.phones[0], "123-4567");
    }

    #[test]
    fn test_struct_to_json() {
        let person = Person {
            name: "Charlie".to_string(),
            age: 40,
            phones: vec!["456-7890".to_string()],
        };

        // Person をJSON文字列にシリアライズ
        let json_str = serde_json::to_string(&person).unwrap();
        println!("シリアライズしたPerson: {}", json_str);

        // JSON文字列から Person にデシリアライズ
        let person2: Person = serde_json::from_str(&json_str).unwrap();
        assert_eq!(person, person2);
    }
}
