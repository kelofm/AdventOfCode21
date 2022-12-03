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


fn getPriority(item: char) -> i64
{
    let lowerCaseBase: i64 = 'a' as i64 - 1;
    let upperCaseBase: i64 = 'A' as i64 - 27;
    return (item as i64) - (if item.is_uppercase() {upperCaseBase} else {lowerCaseBase});
}


fn main()
{
    let mut sum: i64 = 0;
    let mut sets: [HashSet<char>; 3] = Default::default();

    if let Ok(lines) = readLines("input") {
        for (i_line, line) in lines.enumerate() {
            if let Ok(string) = line {
                for item in string.chars() {
                    sets[i_line % 3].insert(item);
                } // for item in line
            } // if line

            if i_line != 0 && (i_line % 3) == 2 {
                for item in sets[0].intersection(&sets[1]).cloned().collect::<HashSet<char>>().intersection(&sets[2]) {
                    sum += getPriority(*item);
                }
                for set in sets.iter_mut() {
                    set.clear();
                }
            }
        } // for line in lines
    } // for lines in readLines

    println!("{}", sum);
}
