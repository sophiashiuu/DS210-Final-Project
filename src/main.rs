use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("layoffs.csv").expect("Failed to open file");
    let reader = BufReader::new(file);
    let _lines = reader.lines();
}


