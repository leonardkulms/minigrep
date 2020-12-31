use std::env;
use minigrep;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (query, filename) = parse_config(&args);
    minigrep::run(query, filename);   
}

fn parse_config(args: &Vec<String>) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}