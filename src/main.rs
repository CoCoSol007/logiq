//! Main entry point for the logik DSL.

use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let Some(path) = args.get(1) else {
        println!("You must provid a file path: logik <File Path>");
        return;
    };

    let file_content_result = fs::read_to_string(path);

    if let Err(error) = file_content_result {
        println!("Error: {}", error);
        return;
    }

    let _file_content = file_content_result.unwrap();
}
