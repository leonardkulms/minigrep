use std::fs;

pub fn run(query: &str, filename: &str) {
    println!("searching for {:#?} in {:#?}", query, filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
