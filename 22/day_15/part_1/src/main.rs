#![allow(non_snake_case)]

// --- STD Imports ---
use std::io::{self, BufRead};
use std::collections::HashSet;
use std::collections::VecDeque;
use std::ops::AddAssign;
use regex::Regex;


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
                self.include(&range);
            }
        }
    }

    fn include(&mut self, range: &(i64,i64))
    {
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
    }

    fn exclude(&mut self, point: i64) {
        match self.contains(point) {
            Some(i_sub) => {
                let target = self.subs[i_sub];
                self.subs.remove(i_sub);
                let mut r = (target.0, point - 1);
                if r.0 <= r.1 {self.include(&r);}
                r = (point + 1, target.1);
                if r.0 <= r.1 {self.include(&r);}
            },
            None => {}
        } // match self.contains(begin)
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
            self.include(&r);
        }
    }
}


fn main()
{
    let mut beacons: HashSet<i64> = Default::default();
    let mut range: Range = Default::default();
    let target: i64 = 2000000;

    if let Ok(pattern) = Regex::new(r"Sensor at x=([\-0-9]+), y=([\-0-9]+): closest beacon is at x=([\-0-9]+), y=([\-0-9]+)") {
        if let Ok(file) = std::fs::File::open("input") {
            for line in io::BufReader::new(&file).lines().flatten() {
                if let Some(m) = pattern.captures(&line) {
                    let coordinates: Vec<i64> = (1..5).map(|i| m.get(i)).flatten().map(|s| s.as_str().parse::<i64>()).flatten().collect();
                    if coordinates[3] == target {
                        beacons.insert(coordinates[2]);
                    }
                    let radius = (coordinates[2] - coordinates[0]).abs() + (coordinates[3] - coordinates[1]).abs();
                    let targetDistance = (coordinates[1] - target).abs();
                    let projectedRadius = radius - targetDistance;
                    if 0 <= projectedRadius {
                        let new = (coordinates[0] - projectedRadius, coordinates[0] + projectedRadius);
                        range.include(&new);
                    } // if 0 < projectedRadius
                } // if pattern matches
            } // for line in file
        } // if file
    } // if pattern

    for xBeacon in beacons {
        range.exclude(xBeacon);
    }
    println!("{}", range.getSize());
}
