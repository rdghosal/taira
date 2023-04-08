use std::fs;
// use serde::{Deserialize, Serialize};
use serde_json::Value;


// #[derive(Serialize, Deserialize)]
// struct TestStruct {
//     test: i32,
// }

fn parse_json(filename: &str) -> Value {
    let data = fs::read_to_string(filename).expect("failed to read file");
    serde_json::from_str(&data).expect("Failed to read json file: {filename}")
}

fn main() {
    // let t: TestStruct = serde_json::from_str(&data).expect("failed to parse json data");
    println!("{}", parse_json("./test.json"));
}
