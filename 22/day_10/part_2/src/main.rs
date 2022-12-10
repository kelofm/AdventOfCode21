#![allow(non_snake_case)]

// --- STD Imports ---
use std::io::{self, BufRead};


fn main()
{
    let mut lines = io::stdin().lock().lines();
    let mut instruction: (i64, i64) = (1, 0);
    let mut register: i64 = 1;

    for cycle in 1.. {
        instruction.0 -= 1;
        if instruction.0 == 0 {
            register += instruction.1;
            match lines.next() {
                Some(Ok(line)) => {
                    if line.starts_with("addx"){
                        if let Ok(diff) = line.chars().skip(5).collect::<String>().parse::<i64>() {
                            instruction = (2, diff);
                        }
                    } else {
                        instruction = (1, 0);
                    }
                }, // Some(line)
                _ => break,
            } // match stdin
        } // if not busy
        let diff = (cycle % 40) - register;
        if 0 <= diff && diff < 3 {
            print!("#");
        } else {
            print!(".");
        }
        if cycle % 40 == 0 {
            print!("\n");
        }
    } // while true
}
