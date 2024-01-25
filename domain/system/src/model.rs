use serde_json::Value;

pub struct Config {
    pub key: String,
    pub value: Value,
    pub version: i64,
}
