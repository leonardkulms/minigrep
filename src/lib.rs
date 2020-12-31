use std::fs;

pub fn run(args: &Vec<String>) {
    let query = &args[1];
    let filename = &args[2];

    println!("searching for {:#?} in {:#?}", query, filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}