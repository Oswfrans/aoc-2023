use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "input.txt"; // Replace with your file path
    let file = File::open(path)?;
    let mut total = 0;

    for line in io::BufReader::new(file).lines() {
        let line = line?;
        let segments = line.split(':').nth(1).unwrap_or("");
        let parts: Vec<&str> = segments.split('|').collect();
        if parts.len() != 2 {
            continue; // Skip invalid lines
        }

        let first_part: Vec<u32> = parts[0]
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        let second_part: Vec<u32> = parts[1]
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        // println!("{}", first_part);
        // println!("{}", second_part);
        let count = first_part
            .iter()
            .filter(|&n| second_part.contains(n))
            .count();
        //println!("{}", count);
        let line_value = if count == 0 { 0 } else { 1 << (count - 1) };
        //println!("{}", line_value);
        total += line_value;
    }

    println!("Total value: {}", total);
    Ok(())
}
