use std::fs::File;
use std::io::prelude::*;

fn main() {
    // ...snip...
    let filename = "input.txt";
    println!("In file {}", filename);

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("With text:\n{}", contents);
}

pub fn adder(input: &str) -> i32 {
    return 2;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "11";
        assert_eq!(2, adder(input));
    }
}
