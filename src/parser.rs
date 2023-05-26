use serde_json::{Result, Value};

pub fn parse_digitus_json(contents: &String) -> Result<Vec<Value>> {
    serde_json::from_str(contents)
}
