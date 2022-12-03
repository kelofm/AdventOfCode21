// --- STD Imports ---
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use core::iter::zip;


fn readLines<P>(fileName: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>,
{
    let file = std::fs::File::open(fileName)?;
    Ok(io::BufReader::new(file).lines())
}


fn getPriority(item: char) -> i64
{
    let lowerCaseBase: i64 = 'a' as i64 - 1;
    let upperCaseBase: i64 = 'A' as i64 - 27;
    return (item as i64) - (if item.is_uppercase() {upperCaseBase} else {lowerCaseBase});
}


fn main()
{
    let mut sum: i64 = 0;

    if let Ok(lines) = readLines("input") {
        'outer: for line in lines {
            if let Ok(string) = line {
                let mut chars = HashSet::new();
                for (forward, reverse) in zip(string.chars(), string.chars().rev()) {
                    // Nasty:
                    // items in the first compartment are positive,
                    // while the ones in the second are negative /\/\(:: vv ::)/\/\
                    // I'm sorry
                    for item in [getPriority(forward), -getPriority(reverse)] {
                        let itemInOtherCompartment = -item;
                        if chars.contains(&itemInOtherCompartment) {
                            sum += item.abs();
                            continue 'outer; // <= yuck
                        }
                        chars.insert(item);
                    }
                } // for char in string
            } // if line
        } // for line in lines
    } // for lines in readLines

    println!("{}", sum);
}
