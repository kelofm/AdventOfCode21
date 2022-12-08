#![allow(non_snake_case)]

// --- STD Imports ---
use std::io::{self, BufRead};
use std::collections::HashSet;


fn insertVisible<It>(it: It, direction: usize, offset: usize, set: &mut HashSet<(usize,usize)>)
where It: Iterator<Item = (usize,usize)>
{
    let mut max: i64 = -1;
    for (i, value) in it {
        if max < value as i64 {
            set.insert(if direction == 0 {(offset,i)} else {(i,offset)});
            max = value as i64;
        } // if max < value
    } // for value in it
}


fn main()
{
    let mut data: Vec<usize> = Default::default();

    if let Ok(file) = std::fs::File::open("input") {
        for maybeLine in io::BufReader::new(&file).lines() {
            if let Ok(line) = maybeLine {
                for height in line.chars().map(|c| match c.to_digit(10) {Some(d) => d, None =>0,} as usize) {
                    data.push(height);
                } // for i, height in line
            } // if line
        } // for line in file
    } // if file

    let mut set: HashSet<(usize,usize)> = Default::default();
    let size: usize = (data.len() as f64).sqrt().round() as usize;
    for offset in 0..size {
        insertVisible(
            data.iter().skip(offset * size).take(size).copied().enumerate(),
            0,
            offset,
            &mut set
        );
        insertVisible(
            data.iter().skip(offset * size).take(size).copied().enumerate().rev(),
            0,
            offset,
            &mut set
        );
    }
    for offset in 0..size {
        insertVisible(
            data.iter().skip(offset).step_by(size).copied().enumerate(),
            1,
            offset,
            &mut set
        );
        insertVisible(
            data.iter().skip(offset).step_by(size).copied().enumerate().rev(),
            1,
            offset,
            &mut set
        );
    }
    println!("{}", set.len());
}
