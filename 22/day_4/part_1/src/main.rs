#![allow(non_snake_case)]

// --- STD Imports ---
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn readLines<P>(fileName: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>
{
    Ok(io::BufReader::new(std::fs::File::open(fileName)?).lines())
}


fn isInside(l: &Vec<i64>, r: &Vec<i64>) -> bool
{
    return ((l[0] < r[0]) != (l[0] <= r[1])) && ((l[1] < r[0]) != (l[1] <= r[1]));
}


fn predicate<'a, T>(mut it: T) -> bool
where T: Iterator<Item = Vec<i64>>
{
    let l = it.next().unwrap();
    let r = it.next().unwrap();
    return isInside(&l, &r) || isInside(&r, &l);
}


fn main()
{
    let mut sum: i64 = 0;

    if let Ok(lines) = readLines("input") {
        for line in lines {
            if let Ok(string) = line {
                sum += predicate(
                    string.split(',')
                          .map(|s| s.split('-').map(|b| b.parse::<i64>().unwrap())
                          .collect::<Vec<i64>>())
                ) as i64;
            } // if string
        } // for line in lines
    } // if lines

    println!("{}", sum);
}
