use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "input.txt";
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut sum_of_ids = 0;
    for (index, line) in reader.lines().enumerate() {
        //println!("index of the line is: {}", index);
        let line = line?;
        if let Some(game_id) = parse_game(&line, index + 1) {
            sum_of_ids += game_id;
        }
    }

    println!("Sum of valid game IDs: {}", sum_of_ids);
    Ok(())
}

fn parse_game(line: &str, game_id: usize) -> Option<usize> {
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;

    let segments = line.split(':').collect::<Vec<&str>>();
    for segment in segments[1].split(';') {
        for color in segment.split(',') {
            println!("color: {}", color);
            let parts: Vec<_> = color.trim().split_whitespace().collect();
            if parts.len() == 2 {
                let count: usize = parts[0].parse().unwrap_or(0);
                match parts[1] {
                    "red" => max_red = max_red.max(count),
                    "green" => max_green = max_green.max(count),
                    "blue" => max_blue = max_blue.max(count),
                    _ => (),
                }
            }
        }
    }

    Some(max_red * max_blue * max_green)
    // if max_red <= 12 && max_green <= 13 && max_blue <= 14 {
    //     //println!("game id: {}", game_id);
    //     Some(game_id)
    // } else {
    //     None
    // }
}
