#![allow(non_snake_case)]

// --- STD Imports ---
use std::io::{self, BufRead};
use std::collections::HashSet;
use regex::Regex;


fn isBounded(position: &(i64,i64), boundaries: &HashSet<(i64,i64)>) -> bool
{
    return 0 < boundaries.iter().filter(|p| position.0 == p.0 && position.1 < p.1).count();
}


fn drop(occupied: &HashSet<(i64,i64)>, boundaries: &HashSet<(i64,i64)>) -> Option<(i64, i64)>
{
    let mut position: (i64, i64) = (500, 0);
    'outer: loop {
        for newPosition in [(position.0, position.1 + 1), (position.0 - 1, position.1 + 1), (position.0 + 1, position.1 + 1)].iter() {
            if !occupied.contains(newPosition) {
                position = newPosition.clone();
                if isBounded(&newPosition, boundaries) {
                    continue 'outer;
                } else {
                    return None;
                }
            }
        } // for newPosition
        return Some(position);
    } // while true
}


fn main()
{
    let mut occupied: HashSet<(i64,i64)> = Default::default();
    let mut boundaries: HashSet<(i64,i64)> = Default::default();

    if let Ok(pattern) = Regex::new("([0-9]+),([0-9]+)") {
        if let Ok(file) = std::fs::File::open("input") {
            for line in io::BufReader::new(&file).lines().flatten() {
                let mut path: Vec<(i64,i64)> = Default::default();


                for m in pattern.captures_iter(&line) {
                    if let (Some(horizontal), Some(vertical)) = (m.get(1), m.get(2)) {
                        if let (Ok(x), Ok(y)) = (horizontal.as_str().parse::<i64>(), vertical.as_str().parse::<i64>()) {
                            path.push((x, y));
                        } // (x, y)
                    } // if (horizontal, vertical)
                } // for match in regex

                let mut begin = path[0].clone();
                for node in path.iter().skip(1) {
                    boundaries.insert(begin);
                    occupied.insert(begin);
                    let direction: (i64, i64) = (
                        (node.0 - begin.0) / (node.0 - begin.0).abs().max(1),
                        (node.1 - begin.1) / (node.1 - begin.1).abs().max(1)
                    );
                    for _ in 0..(node.0 - begin.0).abs().max((node.1 - begin.1).abs()) {
                        begin.0 += direction.0;
                        begin.1 += direction.1;
                        boundaries.insert(begin);
                        occupied.insert(begin.clone());
                    } // for i_step in (end - begin)
                } // for node in path
            } // for line in file
        } // if file
    } // if pattern

    for i_drop in 0.. {
        match drop(&occupied, &boundaries) {
            Some(position) => {
                occupied.insert(position);
            },
            None => {
                println!("{}", i_drop);
                break;
            }
        }
    } // while true
}
