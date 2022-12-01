// --- STD Imports ---
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn readLines<P>(fileName: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>,
{
    let file = std::fs::File::open(fileName)?;
    Ok(io::BufReader::new(file).lines())
}


fn main()
{
    let mut maxes: [u64; 4] = [0; 4];
    let mut tmp: u64 = 0;

    if let Ok(lines) = readLines("input") {
        for line in lines {
            if let Ok(string) = line {
                if !string.is_empty() {
                    tmp += string.parse::<u64>().unwrap();
                } else {
                    maxes[0] = tmp;
                    maxes.sort();
                    tmp = 0;
                }
            }
        } // for line in lines
    } // if lines

    let result: u64 = maxes[1..4].iter().sum();
    println!("{}", result);
}
