// --- STD Imports ---
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp;


fn readLines<P>(fileName: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>,
{
    let file = std::fs::File::open(fileName)?;
    Ok(io::BufReader::new(file).lines())
}


fn main()
{
    let mut max: u64 = 0;
    let mut tmp: u64 = 0;

    if let Ok(lines) = readLines("input") {
        for line in lines {
            if let Ok(string) = line {
                if !string.is_empty() {
                    tmp += string.parse::<u64>().unwrap();
                } else {
                    max = cmp::max(max, tmp);
                    tmp = 0;
                }
            }
        } // for line in lines
    } // if lines

    println!("{}", max);
}
