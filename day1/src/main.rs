fn match_digit(input: &str) -> Option<u32> {
    const NUMBERS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    input.chars().next().unwrap().to_digit(10).or_else(|| {
        NUMBERS.iter().zip(1..).find_map(|(pat, number)| {
            if input.starts_with(pat) {
                Some(number)
            } else {
                None
            }
        })
    })
}

fn digit(line: &str, mut range: impl Iterator<Item = usize>) -> u32 {
    range.find_map(|i| match_digit(&line[i..])).unwrap()
}

fn main() {
    let sum: u32 = include_str!("../input.txt")
        .lines()
        .map(|line| digit(line, 0..line.len()) * 10 + digit(line, (0..line.len()).rev()))
        .sum();
    println!("{sum}");
}
