use log::{debug};
use std::collections::HashMap;
use std::fs;

use serde_json::{Result};

pub fn resolve_config(contract_name: &String) -> Result<HashMap<String, String>> {
    let filename = format!("{}.mapping.json", contract_name);
    let config = fs::read_to_string(filename).expect("nono");

    let map: HashMap<String, String> = serde_json::from_str(&config)?;
    debug!("config map = {:#?}", map);

    Ok(map)
}

/*
    Scratchpad
    let parsed: Value = serde_json::from_str(&config)?;
    let result: Map<String, Value> = parsed.as_object().unwrap().clone();

    print!("{:?}", result);
*/
