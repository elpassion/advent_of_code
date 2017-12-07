pub fn number_of_steps(cell_number: i32) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        for &(x, y) in [(1, 0), (2, 1), (3, 2), (4, 1), (5, 2), (6, 1), (7, 2)].iter() {
            assert_eq!(y, number_of_steps(x));
        }
    }
}
