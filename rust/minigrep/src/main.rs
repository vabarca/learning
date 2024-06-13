extern crate argparse;

use argparse::{ArgumentParser, Store};
use std::fs;

fn main() {
    let config: Config;


    let mut file: String = "".to_string();
    let mut query: String = "".to_string();
    {  // this block limits scope of borrows by ap.refer() method
        let mut ap: ArgumentParser = ArgumentParser::new();
        ap.set_description("Minigrep app emulates a grep command but in a more simple way");
        ap.refer(&mut file)
            .add_argument("file", Store,
            "File to open").required();
        ap.refer(&mut query)
            .add_argument("query", Store,
            "String to search").required();
        ap.parse_args_or_exit();
    }

    println!("File: {}", file);
    println!("Query: {}", query);

    let content = fs::read_to_string(file).expect("Error reading the file");

    println!("{}", &content[..30]);
}

struct Config{
    filename: String,
    query: String,
}
