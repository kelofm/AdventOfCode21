#![allow(non_snake_case)]

// --- STD Imports ---
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
use std::collections::VecDeque;


fn readLines<P>(fileName: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>
{
    Ok(io::BufReader::new(std::fs::File::open(fileName)?).lines())
}


fn main()
{
    let mut stacks: Vec<VecDeque<char>> = Default::default();

    let cratePattern = Regex::new(r"(?:(\[[A-Z]\])|(   ))(?: |\n)?").unwrap();
    let instructionPattern = Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
    if let Ok(lines) = readLines("input") {
        for line in lines {
            if let Ok(string) = line {
                for (i_c, c) in cratePattern.captures_iter(&string).enumerate() {
                    for sub in c.iter().next() {
                        for ssub in sub {
                            let value = ssub.as_str().chars().into_iter().nth(1).unwrap();
                            if value != ' ' {
                                if stacks.len() <= i_c {
                                    stacks.resize(i_c + 1, VecDeque::from([]));
                                }
                                stacks[i_c].push_back(value);
                            } // if value
                        } // for ssub
                    } // for sub
                } // for capture
                for c in instructionPattern.captures_iter(&string) {
                    let numberOfCrates = c.get(1).unwrap().as_str().parse::<usize>().unwrap();
                    let from = c.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
                    let to = c.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1;

                    let mv: Vec<char> = stacks.iter().nth(from).unwrap().iter().take(numberOfCrates).copied().collect();
                    stacks.iter_mut().nth(from).unwrap().drain(0..numberOfCrates);
                    let mut stack = stacks.iter_mut().nth(to).unwrap();
                    //stack.insert(0, it_from); // <== great, there's no way to prepend a deque, awesome
                    for v in mv.iter().rev().copied() {
                        stack.insert(0, v);
                    }
                } // for capture
            } // if line
        } // for line in lines
    } // if lines

    for stack in stacks {
        print!("{}", stack.front().unwrap());
    }
    println!();
}
