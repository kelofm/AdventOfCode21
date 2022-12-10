#![allow(non_snake_case)]

// --- STD Imports ---
use std::io::{self, BufRead};


fn main()
{
    let mut cycle: i64 = 1;
    let mut register: i64 = 1;
    let mut sum: i64 = 0;

    for maybeLine in io::stdin().lock().lines() {
        if let Ok(line) = maybeLine {
            //println!("{}", line);
            let mut step: i64 = 0;
            let mut diff: i64 = 0;
            if line.starts_with("noop") {
                step = 1;
            } else if line.starts_with("addx") {
                step = 2;
                if let Ok(arg) = line.chars().skip(5).collect::<String>().parse::<i64>() {
                    diff = arg;
                } // if arg in instruction
            } // if addx in line
            if sum == 0 && (cycle == 20 || (cycle == 19 && step == 2)) {
                sum += 20 * register;
                cycle -= 20;
            } else {
                if cycle % 40 == 0 || (cycle % 40 == 39 && step == 2) {
                    let sample = (cycle as f64 / 40.0).round() as i64 * 40 + 20;
                    sum += sample * register;
                } // if sample
            }
            cycle += step;
            register += diff;
        } // if Ok(line)
    } // for maybeLin in stdin

    println!("{}", sum);
}
