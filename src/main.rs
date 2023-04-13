use serde_json::{Map, Value};
use std::fs;

fn json_to_map(filename: &str) -> Map<String, Value> {
    let data = fs::read_to_string(filename).expect("failed to read file");
    serde_json::from_str(&data).expect("Failed to read json file: {filename}")
}

fn parse_json_map(
    current: &mut Map<String, Value>,
    parent: &mut Option<&mut Map<String, Value>>,
    prefix: Option<&str>,
) {
    for (k, v) in current.clone().iter_mut() {
        match v {
            Value::Object(next) => {
                parse_json_map(next, &mut Some(current), Some(&k.to_string()));
                current.remove(k);
            }
            Value::Array(array) => {
                for (i, element) in array.iter_mut().enumerate() {
                    if let Value::Object(next) = element {
                        parse_json_map(next, &mut Some(current), Some(&i.to_string()));
                        current.remove(k);
                    } else {
                        println!("error!");
                    }
                }
            }
            _ => {
                if let Option::Some(p) = parent {
                    println!("inserting {}: {}", k, v);
                    p.insert(
                        format!("{}_{}", prefix.unwrap_or(k), k.to_string()),
                        v.clone(),
                    );
                }
            }
        }
    }
}

fn main() {
    let mut map = json_to_map("./test.json");
    parse_json_map(&mut map, &mut None, None);
    for k in map.keys() {
        println!("{}", k);
        println!("{}", map[k]);
    }
}
