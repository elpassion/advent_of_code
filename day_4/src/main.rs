use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("input.text").expect("file not found");
    let mut input = String::new();
    let mut valid_lines = 0;

    f.read_to_string(&mut input).expect("can't read file");

    for line in input.lines() {
        if valid_line(line) { valid_lines += 1; }
    }

    println!("{}", valid_lines);
}

fn valid_line(line: &str) -> bool {
    let mut words_count = HashMap::new();

    for word in line.split_whitespace() {
        let counter = words_count.entry(word).or_insert(0);
        *counter += 1;
    }

    !words_count.values().any(|&x| x > 1)
}
