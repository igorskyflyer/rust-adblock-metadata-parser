mod adblock_metadata;

// use std::fs;
use self::adblock_metadata::AdBlockMetaData;

fn parse_metadata(_path: &str) -> AdBlockMetaData {
    // let result: std::str::Lines = fs::read_to_string(path)
    //     .expect("Error in reading the file")
    //     .lines();
    AdBlockMetaData {
        author: String::from("Me"),
    }
}

fn main() {
    let meta = parse_metadata("./samples/valid.meta.txt");
    println!("{}", meta.author);
}
