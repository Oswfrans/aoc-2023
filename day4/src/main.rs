use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "input.txt"; // Replace with your file path
    let file = File::open(path)?;
    //let lines: Vec<_> = io::BufReader::new(file).lines().collect::<Result<_, _>>()?;

    let lines: Vec<_> = io::BufReader::new(file).lines().collect::<Result<_, _>>()?;

    let mut queue: VecDeque<usize> = (0..lines.len()).collect();
    let mut winnings_cache: HashMap<usize, usize> = HashMap::with_capacity(lines.len());

    let mut sum = 0;
    while let Some(line_n) = queue.pop_front() {
        sum += 1;

        let line_win_amount = *winnings_cache
            .entry(line_n)
            .or_insert_with(|| check_correct_nums(&lines[line_n]));

        for r in 1..=line_win_amount {
            queue.push_back(line_n + r);
        }
    }
    println!("Total value: {}", sum);
    Ok(())
}

fn check_correct_nums(line: &str) -> usize {
    let (winning, guesses) = parse_line(line);

    let guesses = str_to_num_iter(guesses);
    let winning_set = str_to_num_iter(winning).collect::<HashSet<_>>();
    guesses.filter(|e| winning_set.contains(e)).count()
}

fn str_to_num_iter(s: &str) -> impl Iterator<Item = usize> + '_ {
    s.split_whitespace().filter_map(|s| s.parse::<usize>().ok())
}

fn parse_line(line: &str) -> (&str, &str) {
    let mut parts = line.split('|').map(str::trim);
    (parts.next().unwrap(), parts.next().unwrap())
}
