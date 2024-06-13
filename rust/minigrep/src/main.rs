extern crate argparse;

use argparse::{ ArgumentParser, Store };
use std::fs;
use std::env;
use minigrep::*;

fn main() {
    let mut filename: String = "".to_string();
    let mut query: String = "".to_string();
    {
        // this block limits scope of borrows by ap.refer() method
        let mut ap: ArgumentParser = ArgumentParser::new();
        ap.set_description("Minigrep app emulates a grep command but in a more simple way");
        ap.refer(&mut filename).add_argument("file", Store, "File to open").required();
        ap.refer(&mut query).add_argument("query", Store, "String to search").required();
        ap.parse_args_or_exit();
    }

    let cfg: Config = Config { filename, query };

    println!("File: {}", cfg.filename);
    println!("Query: {}", cfg.query);

    match env::current_exe() {
        Ok(exe_path) => println!("Path of this executable is: {}", exe_path.display()),
        Err(e) => println!("failed to get current exe path: {e}"),
    }

    let content = fs
        ::read_to_string(&cfg.filename)
        .expect(&format!("Error reading the file {}", cfg.filename)[..]);
    let result: Vec<&str> = analyze_file(&cfg, &content);

    print_result(&result);
}
