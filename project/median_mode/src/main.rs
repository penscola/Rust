mod median_and_mode;
use crate::median_and_mode::me_mo::median_and_mode;

fn main() {
    let numbers = vec![1, 2, 2, 3, 4, 4, 4, 5];
    let (median, mode) = median_and_mode(numbers); // Passing ownership instead of reference
    println!("Median: {}, Mode: {}", median, mode);
}
