use std::fs;

// use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::{Map, Value};

// #![feature(type_name_of_val)]
// use std::any::type_name_of_val;


// #[derive(Serialize, Deserialize)]
// struct TestStruct {
//     test: i32,
// }

fn json_to_map(filename: &str) -> Map<String, Value> {
    let data = fs::read_to_string(filename).expect("failed to read file");
    serde_json::from_str(&data).expect("Failed to read json file: {filename}")
}

fn parse_json_map(parent: &mut Map<String, Value>, current_idx: Option<usize>) {
    for (k, v) in parent.iter() {
        if v.is_object() {
            let mut next: Map<String, Value> = v.as_object().unwrap().clone();
            parse_json_map(&mut next, None);
        }
        else if v.is_array() {
            for (i, element) in v.as_array().unwrap().iter().enumerate() {
                let mut next: Map<String, Value> = element.as_object().expect("not an obj!").clone();
                parse_json_map(&mut next, Some(i));
            }
        }
        else {
            continue;
        }
    }
}




fn main() {
    // let t: TestStruct = serde_json::from_str(&data).expect("failed to parse json data");
    //
    let mut map = json_to_map("./test.json");
    parse_json_map(&mut map, None);

}
