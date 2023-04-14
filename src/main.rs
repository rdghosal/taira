use serde_json::{Map, Value};
use std::{env, fs};

fn json_to_map(filename: &str) -> Map<String, Value> {
    log::info!("reading file: {}", filename);
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
                        let prefix = format!("{}[{}]", k.to_string(), i.to_string());
                        parse_json_map(next, &mut Some(current), Some(&prefix));
                        current.remove(k);
                    } else {
                        log::error!("reached invalid child element");
                    }
                }
            }
            _ => {
                if let Option::Some(p) = parent {
                    let fkey = format!("{}_{}", prefix.unwrap_or(&""), k.to_string());
                    if !p.contains_key(&fkey) {
                        // fixme: producing duplicate logs...
                        log::info!("inserting {}: {}", fkey, v);
                        p.insert(fkey, v.clone());
                    } else {
                        log::warn!("skipping insert duplicate key {fkey}: {v}");
                    }
                }
            }
        }
    }
}

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    let mut map = json_to_map(&args[1]);
    parse_json_map(&mut map, &mut None, None);
    fs::write(
        "./results.json",
        serde_json::to_string_pretty(&map).unwrap(),
    )
    .expect("failed to save results!");
    log::info!("done!");
}
