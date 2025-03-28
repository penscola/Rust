
use std::collections::HashMap;

pub fn median_and_mode(mut numbers: Vec<i32>) -> (f64, i32) {
    numbers.sort(); // Now sorting without cloning

    // Median calculation remains the same
    let len = numbers.len();
    let median = if len % 2 == 0 {
        (numbers[len / 2 - 1] + numbers[len / 2]) as f64 / 2.0
    } else {
        numbers[len / 2] as f64
    };

    // Mode calculation remains the same
    let mut occurrences = HashMap::new();
    for &num in &numbers {
        *occurrences.entry(num).or_insert(0) += 1;
    }

    let mode = occurrences
        .iter()
        .max_by_key(|&(_, count)| count)
        .map(|(num, _)| *num)
        .unwrap_or(0);

    (median, mode)
}
