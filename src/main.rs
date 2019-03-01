extern crate treexml;
extern crate serde_json;
extern crate node2object;
extern crate exitcode;

use std::env;
use std::fs;
use std::mem;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("XML input file needed");
        std::process::exit(exitcode::CONFIG);
    }

    let file_name = args.get(1).unwrap();
    let input_str = fs::read_to_string(file_name)
        .expect("Something went wrong reading the file");

    let dom_root = treexml::Document::parse(input_str.as_bytes()).unwrap().root.unwrap();
    mem::forget(input_str);

    let result = serde_json::Value::Object(node2object::node2object(&dom_root));
    mem::forget(dom_root);
    println!("{}", result);
}