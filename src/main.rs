use std::fs;
use std::env;

fn main() {

    println!("\n----------Starting Markdown to HTML----------\n");

    let args: Vec<String> = env::args().collect();

    // The first argument at runtime
    let file_path = &args[1];

    println!("Reading from file_path: {file_path}\n");

    let contents = fs::read_to_string(file_path).expect("unable to read the file");

    println!("Succesfully read contents:\n{contents}\n");



}
