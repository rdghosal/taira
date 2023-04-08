use std::fs;

// use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::Value;

// #![feature(type_name_of_val)]
// use std::any::type_name_of_val;


// #[derive(Serialize, Deserialize)]
// struct TestStruct {
//     test: i32,
// }

fn parse_json(filename: &str) -> HashMap<String, Value> {
    let data = fs::read_to_string(filename).expect("failed to read file");
    serde_json::from_str(&data).expect("Failed to read json file: {filename}")
}

fn main() {
    // let t: TestStruct = serde_json::from_str(&data).expect("failed to parse json data");
    let parsed = parse_json("./test.json");
    let mut keys: Vec<_> = parsed.keys().clone().collect();
    for (_, value) in parsed.iter() {
        if value.is_object() {
            // println!("{:?}", value);
        }
        else if value.is_array() {
            // println!("{:?}", value);
        }
    }
    for k in keys.iter() {
        println!("{}", &k);
    }
}
