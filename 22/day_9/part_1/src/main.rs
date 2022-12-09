#![allow(non_snake_case)]

// --- STD Imports ---
use std::io::{self, BufRead};
use std::collections::HashSet;


type Position = (i64,i64);          // (x, y)
type State = (Position,Position);   // (head, tail)
type Instruction = (Position,i64);  // (direction, stepCount)


fn step(oldState: &State, instruction: &Instruction, set: &mut HashSet<Position>) -> State
{
    let mut head = oldState.0.clone();
    let mut tail = oldState.1.clone();
    for _ in 0..instruction.1 {
        head.0 += instruction.0.0;
        head.1 += instruction.0.1;
        let enable: i64 = (1 < (head.0 - tail.0).abs() || 1 < (head.1 - tail.1).abs()) as i64;
        tail.0 += enable * (head.0 - tail.0).signum();
        tail.1 += enable * (head.1 - tail.1).signum();
        set.insert(tail);
    } // for _ in instruction
    return (head, tail);
}


fn main()
{
    let mut set: HashSet<Position> = Default::default();
    let mut state: State = Default::default();
    set.insert(state.1);
    if let Ok(file) = std::fs::File::open("input") {
        for line in io::BufReader::new(&file).lines().flatten() {
            if let Some(c) = line.chars().next() {
                let mut instruction: Instruction = Default::default();
                match c {
                    'R' => instruction.0.0 = 1,
                    'L' => instruction.0.0 = -1,
                    'U' => instruction.0.1 = 1,
                    'D' => instruction.0.1 = -1,
                     _  => panic!("Unrecognized instruction '{}'", c),
                } // match instruction
                let stepCountString = line.chars().skip(2).collect::<String>();
                if let Ok(stepCount) = stepCountString.parse::<i64>() {
                    instruction.1 = stepCount;
                    state = step(&state, &instruction, &mut set);
                } // if stepCount
            } // if instruction
        } // for line in file
    } else {
        println!("Failed to open input");
    }
    println!("{}", set.len());
}
