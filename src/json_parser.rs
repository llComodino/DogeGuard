/*
 * JSON Parser
 *
 * This module is responsible for parsing JSON files.
 * It is used by the main module to parse the database
 *
 * format:
 * {
 *      "website": {
 *          "name": "website name",
 *          "username": "username",
 *          "password": "password",
 *          "url": "website url"
 *      }
 *      ...
 * }
 *
 */

use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use serde_json::Value;

pub fn parse_json_file(filename: &str) -> HashMap<String, HashMap<String, String>> {
    let mut file = File::open(filename).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading file");

    let json: Value = serde_json::from_str(&contents).expect("Error parsing JSON");

    let mut database: HashMap<String, HashMap<String, String>> = HashMap::new();

    for (key, value) in json.as_object().unwrap() {
        let mut website: HashMap<String, String> = HashMap::new();
        for (k, v) in value.as_object().unwrap() {
            website.insert(k.to_string(), v.as_str().unwrap().to_string());
        }
        database.insert(key.to_string(), website);
    }

    database
}
