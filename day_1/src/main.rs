use std::fs::File;
use std::io::prelude::*;

fn main() {
    let filename = "input.txt";
    println!("In file {}", filename);

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let result = adder(contents);
    println!("With text:\n{}", result);
}

pub fn adder(input: String) -> u32 {
    let mut sum = 0;

    let numbers: Vec<u32> = input.chars()
        .filter_map(|c| c.to_digit(10))
        .collect();

    for n in 0..numbers.len() {
        let mut next_index = n + 1;

        if n == numbers.len() - 1 {
            next_index = 0;
        }

        if numbers[n] == numbers[next_index] {
            sum += numbers[n];
        }
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_one() {
        assert_eq!(1, adder(String::from("1")));
    }

    #[test]
    fn test_1122() {
      assert_eq!(3, adder(String::from("1122")));
    }

    #[test]
    fn test_1111() {
      assert_eq!(4, adder(String::from("1111")));
    }

    #[test]
    fn test_1234() {
      assert_eq!(0, adder(String::from("1234")));
    }

    #[test]
    fn test_91212129() {
      assert_eq!(9, adder(String::from("91212129")));
    }

    #[test]
    fn test_1() {
        assert_eq!(3, adder(String::from("111")));
    }

    #[test]
    fn test_2() {
        assert_eq!(4, adder(String::from("22")));
    }
}
