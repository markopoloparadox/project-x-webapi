use std::collections::HashMap;

use serde_json::json;

pub struct Database {
    pub database: HashMap<String, serde_json::Value>,
}

pub struct AAA();

impl Database {
    pub fn new() -> Self {
        let mut database = HashMap::new();
        let v: serde_json::Value = json!([
            {
                "family": "Ana",
                "name": "A"
            },
            {
                "family": "Banana",
                "name": "B"
            },
            {
                "family": "Ana",
                "name": "C"
            }
        ]);

        database.insert("test".to_string(), v);
        Self { database }
    }

    pub fn query(&self, command: &str) -> serde_json::Value {
        let mut command = command.trim();
        let mut tokens: Vec<&str> = command.split(' ').rev().collect();

        let token = tokens.pop().unwrap();
        assert_eq!(token, "GET");

        let token = tokens.pop().unwrap();
        assert_eq!(token, "ALL");

        let token = tokens.pop().unwrap();
        assert_eq!(token, "FROM");

        let file_name = tokens.pop().unwrap();
        let db_value = self.database.get(file_name).unwrap();
        let db_value = db_value.as_array().unwrap();

        let token = tokens.pop().unwrap();
        assert_eq!(token, "WHERE");

        let field = tokens.pop().unwrap();

        let token = tokens.pop().unwrap();
        assert_eq!(token, "=");

        let expected_value = tokens.pop().unwrap();

        let all_items: Vec<serde_json::Value> = db_value
            .iter()
            .filter(|x| {
                let value = x.as_object().unwrap();
                let field_value = value.get(field).unwrap();
                field_value.as_str().unwrap() == expected_value
            })
            .map(|x| x.clone())
            .collect();

        serde_json::Value::Array(all_items)
    }
}
