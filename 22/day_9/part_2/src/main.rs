#![allow(non_snake_case)]

// --- STD Imports ---
use std::io::{self, BufRead};
use std::collections::HashSet;


type Position = (i64,i64);          // (x, y)
type State = [Position;10];         // (head, .., tail)
type Instruction = (Position,i64);  // (direction, stepCount)


fn step(oldState: &State, instruction: &Instruction, set: &mut HashSet<Position>) -> State
{
    let mut state = oldState.clone();
    for _ in 0..instruction.1 {
        state[0].0 += instruction.0.0;
        state[0].1 += instruction.0.1;
        for (i_head, i_tail) in (0..state.len() - 1).zip(1..state.len()) {
            let head = state[i_head];
            let mut tail = &mut state[i_tail];
            let enable: i64 = (1 < (head.0 - tail.0).abs() || 1 < (head.1 - tail.1).abs()) as i64;
            tail.0 += enable * (head.0 - tail.0).signum();
            tail.1 += enable * (head.1 - tail.1).signum();
        } // for head, tail in state
        set.insert(state[state.len() - 1]);
    } // for _ in instruction
    return state;
}


fn main()
{
    let mut set: HashSet<Position> = Default::default();
    let mut state: State = Default::default();
    set.insert(state[0]);
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
