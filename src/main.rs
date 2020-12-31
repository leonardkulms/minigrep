use std::env;
use minigrep::run as run;

fn main() {
    let args: Vec<String> = env::args().collect();
    run(&args);   
}