use serde_json::{Value, Error};

pub struct JsonParser;

impl JsonParser {
    pub fn parse(&self, input: &str) -> Result<Value, Error> {
        serde_json::from_str(input)
    }

    pub fn stringify(&self, value: &Value) -> String {
        value.to_string()
    }
}