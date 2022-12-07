#![allow(non_snake_case)]

// --- STD Imports ---
use std::io::{self, BufRead};
use std::collections::HashMap;
use regex::Regex;


fn main()
{
    let mut map: HashMap<String,usize> = Default::default();
    let mut curDir: Vec<String> = Default::default();

    // group 0: all
    // group 1: cd (dir)
    // group 3: (size) (name)
    match Regex::new(r"(\$ cd ([\w\./]+))|(([0-9]+) ([\w\.]+))") {
        Ok(pattern) => {
            match std::fs::File::open("input") {
                Ok(file) => {
                    for maybeLine in io::BufReader::new(file).lines() {
                        match maybeLine {
                            Ok(line) => {

                                match pattern.captures(&line) {
                                    Some(captures) => {
                                        if let Some(_) = captures.get(1) {
                                            // cd (dir)
                                            if let Some(dir) = captures.get(2) {
                                                let directory = dir.as_str();
                                                if directory == "/" {
                                                    curDir.clear();
                                                    curDir.push(directory.clone().to_string());
                                                } else if directory == ".."{
                                                    curDir.pop();
                                                } else {
                                                    curDir.push(directory.clone().to_string());
                                                }
                                            } // cd dir
                                        } else if let Some(sizeString) = captures.get(4) {
                                            // (size) (name)
                                            if let Ok(size) = sizeString.as_str().parse::<usize>() {
                                                let mut path: String = Default::default();
                                                for dir in curDir.iter() {
                                                    path += dir;
                                                    *map.entry(path.clone()).or_insert(0) += size;
                                                } // for dir in curDir
                                            }
                                        } // (size) (name)
                                    },
                                    None => {}, // no match
                                } // match captures

                            },
                            Err(error) => println!("Failed to read line: {}", error.to_string()),
                        } // match maybeLine
                    } // for maybeLine in fileReader
                }, // Ok(file)
                Err(error) => println!("{}", error.to_string()),
            } // match open("input")
        }, // Ok(pattern)
        Err(error) => println!("Regex compilation failed: {}", error.to_string()),
    } // match regex

    if let Some(total) = map.get("/") {
        let storage: usize = 70000000;
        let updateSize: usize = 30000000;
        let free: usize = total - (storage - updateSize);
        let mut choice: usize = usize::max_value();
        for (_, size) in map {
            if free <= size.clone() && size < choice {
                choice = size;
            }
        } // for name, size in map
        println!("{}", choice);
    } else {println!("Root not in map!");}
}
