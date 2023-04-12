use std::{fs, borrow::BorrowMut};

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

fn parse_json_map(
    current: &mut Map<String, Value>,
    parent: &mut Option<Map<String, Value>>, 
    current_idx: Option<usize>) -> Map<String, Value> {

    for (k, v) in current.clone().iter_mut() {
        match v {
            Value::Object(next) => { parse_json_map(next, &mut Some(current.to_owned()), None); }
            Value::Array(array) => {
                for (i, element) in array.iter_mut().enumerate() {
                    if let Value::Object(next) = element {
                        parse_json_map(next, &mut Some(current.to_owned()), Some(i));
                    } else {
                        println!("error!");
                    }
                };
            },
            // todo
            _ => {
                if let Option::Some(p) = parent {
                    p.insert(k.to_string(), v.clone());
                }
            }
        }
    }

    current.to_owned()
}

fn main() {
    // let t: TestStruct = serde_json::from_str(&data).expect("failed to parse json data");
    let mut map = json_to_map("./test.json");
    map = parse_json_map(&mut map, &mut None, None);
    for k in map.keys() {
        println!("{}", k);
    }
}
