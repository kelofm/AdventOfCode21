#![allow(non_snake_case)]

// --- STD Imports ---
use std::io::{self, BufRead};


fn setScores<'l,It>(it: It)
where It: Iterator<Item = (usize, &'l mut usize)>
{
    let mut memory: Vec<usize> = (0..10).map(|_| 0).collect();
    for (value, score) in it {
        *score = memory[value];
        for v in memory.iter_mut().take(value + 1) {*v = 1;}
        for v in memory.iter_mut().skip(value + 1) {*v += 1;}
    }
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

    let mut arr: [usize;4] = Default::default();
    arr[1] = 1;

    let size: usize = (data.len() as f64).sqrt().round() as usize;
    let mut scores: Vec<[usize;4]> = data.iter().map(|_| [1; 4]).collect();
    for offset in 0..size {
        setScores(data.iter().copied().zip(scores.iter_mut().map(|arr| &mut arr[0])).skip(offset * size).take(size));
        setScores(data.iter().copied().zip(scores.iter_mut().map(|arr| &mut arr[1])).skip(offset * size).take(size).rev());
        setScores(data.iter().copied().zip(scores.iter_mut().map(|arr| &mut arr[2])).skip(offset).step_by(size).take(size));
        setScores(data.iter().copied().zip(scores.iter_mut().map(|arr| &mut arr[3])).skip(offset).step_by(size).take(size).rev());
    }

    if let Some(max) = scores.iter().map(|arr| arr.iter().product::<usize>()).max() {
        println!("{}", max);
    }
}
