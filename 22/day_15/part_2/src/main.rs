#![allow(non_snake_case)]

// --- STD Imports ---
use std::io::{self, BufRead};
use std::collections::VecDeque;
use std::ops::AddAssign;
use regex::Regex;


const RANGE: (i64, i64) = (0, 4000000);


struct Range
{
    subs: VecDeque<(i64,i64)>
}


impl Range
{
    fn contains(&self, value: i64) -> Option<usize>
    {
        for (i_sub, sub) in self.subs.iter().enumerate() {
            if sub.0 <= value && value <= sub.1 {
                return Some(i_sub);
            }
        }
        return None;
    }

    fn merge(&mut self)
    {
        let size = self.subs.len();
        for _ in 0..size {
            if let Some(range) = self.subs.pop_front() {
                self.include(range);
            }
        }
    }

    fn include(&mut self, mut range: (i64,i64))
    {
        range = (range.0.max(RANGE.0), range.1.min(RANGE.1));
        if range.0 <= range.1 {
            match self.contains(range.0) {
                Some(i_sub) => {
                    self.subs[i_sub].1 = self.subs[i_sub].1.max(range.1);
                    self.merge();
                },
                None => {
                    match self.contains(range.1) {
                        Some(i_sub) => {
                            self.subs[i_sub].0 = self.subs[i_sub].0.min(range.0);
                            self.merge();
                        },
                        None => {
                            self.subs.push_back(range.clone());
                        } // !self.contains(end)
                    } // match end
                } // !self.contains(begin)
            } // match begin
        } // if non-degenerate
    }

    fn exclude(&mut self, range: (i64, i64))
    {
        let mut newRange: Range = Default::default();
        for sub in self.subs.iter() {
            if sub.0 < range.0 {
                let fraction = (sub.0, sub.1.min(range.0 - 1));
                newRange.include(fraction);
            }
            if range.1 < sub.1 {
                let fraction = (sub.0.max(range.1 + 1), sub.1);
                newRange.include(fraction);
            }
        }
        self.subs = newRange.subs;
    }

    fn getSize(&mut self) -> usize
    {
        self.merge();
        return self.subs.iter().map(|r| r.1 - r.0 + 1).sum::<i64>() as usize;
    }
}


impl Default for Range
{
    fn default() -> Range
    {
        return Range{subs: Default::default()};
    }
}


impl AddAssign for Range
{
    fn add_assign(&mut self, right: Self)
    {
        for r in right.subs {
            self.include(r);
        }
    }
}


fn main()
{
    let mut pairs: Vec<((i64,i64),(i64,i64))> = Default::default();

    if let Ok(pattern) = Regex::new(r"Sensor at x=([\-0-9]+), y=([\-0-9]+): closest beacon is at x=([\-0-9]+), y=([\-0-9]+)") {
        if let Ok(file) = std::fs::File::open("input") {
            for line in io::BufReader::new(&file).lines().flatten() {
                if let Some(m) = pattern.captures(&line) {
                    let coordinates: Vec<i64> = (1..5).map(|i| m.get(i)).flatten().map(|s| s.as_str().parse::<i64>()).flatten().collect();
                    pairs.push(((coordinates[0], coordinates[1]), (coordinates[2], coordinates[3])));
                } // if pattern matches
            } // for line in file
        } // if file
    } // if pattern

    for target in RANGE.0..RANGE.1 + 1 {
        let mut range = Range {subs: VecDeque::from([(RANGE.0, RANGE.1)])};
        for (sensor, beacon) in pairs.iter() {
            let radius = (beacon.0 - sensor.0).abs() + (beacon.1 - sensor.1).abs();
            let targetDistance = (sensor.1 - target).abs();
            let projectedRadius = radius - targetDistance;
            if 0 <= projectedRadius {
                let new = (sensor.0 - projectedRadius, sensor.0 + projectedRadius);
                range.exclude(new);
                if range.getSize() == 0 {
                    break;
                }
            } // if 0 < projectedRadius
        } // for pair in pairs
        if 0 < range.getSize() as usize {
            println!("{}", range.subs[0].0 * RANGE.1 + target);
            break;
        }
    } // for target in domain
}
