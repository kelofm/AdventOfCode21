// --- STD Imports ---
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;


fn readLines<P>(fileName: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>,
{
    let file = std::fs::File::open(fileName)?;
    Ok(io::BufReader::new(file).lines())
}


fn getScore(opponent: char, mut player: char) -> i64
{
    let map: String = "ABCA".to_string();
    let numberOfOptions: i64 = map.chars().count() as i64 - 1;
    let opponentIndex: i64 = map.chars().position(|choice| choice == opponent).unwrap() as i64;

    // Convert XYZ => ABC
    player = map.chars().nth(
        if player == 'X' {
            (opponentIndex - 1 + numberOfOptions) % numberOfOptions
        } else if player == 'Y' {
            opponentIndex
        } else {
            (opponentIndex + 1) % numberOfOptions
        } as usize
    ).unwrap();

    let game: String = [opponent, player].iter().collect();
    return if opponent == player {
        3 // draw
    } else if map.contains(&game) {
        6 // win
    } else {
        0 // lose
    } + player as i64 - 'A' as i64 + 1 /* score from player choice */
}


fn main()
{
    let mut score: i64 = 0;

    if let Ok(lines) = readLines("input") {
        for line in lines {
            if let Ok(string) = line {
                score += getScore(string.chars().next().unwrap(),
                                  string.chars().last().unwrap());
            } // if line
        } // for line in lines
    } // for lines in readLines

    println!("{}", score);
}
