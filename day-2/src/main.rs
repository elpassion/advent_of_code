use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let file = File::open("spreed.txt").expect("Unable to open");
    let mut checksum = 0;

    for line in BufReader::new(file).lines() {
	let numbers: Vec<i32> = line.unwrap()
           .split_whitespace()
           .map(|s| s.parse().unwrap())
           .collect();
	let min = numbers.iter().min().unwrap();
	let max = numbers.iter().max().unwrap();
	checksum += max - min
    }
    println!("Checksum: {}", checksum);
}
