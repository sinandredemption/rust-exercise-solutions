use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("numbers.txt")?;
    let reader = BufReader::new(f);

    // Write your iterator chain here
    let sum_of_odd_numbers: i32 = reader
        .lines()
        .filter_map(|s| s.ok())
        .filter_map(|a| a.parse::<i32>().ok())
        .filter(|a| a % 2 != 0)
        .sum::<i32>();

    println!("sum: {}", sum_of_odd_numbers);

    assert_eq!(sum_of_odd_numbers, 31);
    Ok(())
}
