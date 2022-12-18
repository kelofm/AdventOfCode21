#![allow(non_snake_case)]

// --- STD Imports ---
use std::io::{self, BufRead};
use std::cmp::Ordering;


enum Item {
    Int(i8),
    List(Vec<Item>)
}

impl std::fmt::Display for Item
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match &self {
            Item::Int(value) => {
                print!("{}", value);
            },
            Item::List(list) => {
                print!("[");
                for item in list {
                    match std::fmt::Display::fmt(&item, f) {
                        _ => {}
                    }
                }
                print!("]");
            },
        }
        return Ok(());
    }
}


impl PartialEq for Item
{
    fn eq(&self, right: &Self) -> bool
    {
        match self {
            Item::Int(value) => {
                match right {
                    Item::Int(rightValue) => {
                        return value == rightValue;
                    },
                    Item::List(_) => {
                        let left = Item::List(Vec::from([Item::Int(value.clone())]));
                        return &left == right;
                    }
                }
            },
            Item::List(list) => {
                match right {
                    Item::Int(rightValue) => {
                        let rightList = Item::List(Vec::from([Item::Int(rightValue.clone())]));
                        return self == &rightList;
                    },
                    Item::List(rightList) => {
                        if list.len() == rightList.len() {
                            for (l, r) in list.iter().zip(rightList.iter()) {
                                if l != r {
                                    return false;
                                }
                            }
                            return true;
                        } else {
                            return false;
                        }
                    }
                }
            }
        }
    }
}


impl Eq for Item {}


impl PartialOrd for Item
{
    fn partial_cmp(&self, right: &Self) -> Option<Ordering> {
        if self == right {
            return Some(Ordering::Equal);
        } else {
            match self {
                Item::Int(value) => {
                    match right {
                        Item::Int(rightValue) => {
                            return value.partial_cmp(rightValue);
                        },
                        Item::List(_) => {
                            let left = Item::List(Vec::from([Item::Int(value.clone())]));
                            return left.partial_cmp(right);
                        }
                    }
                },
                Item::List(list) => {
                    match right {
                        Item::Int(rightValue) => {
                            let rightList = Item::List(Vec::from([Item::Int(rightValue.clone())]));
                            return self.partial_cmp(&rightList);
                        },
                        Item::List(rightList) => {
                            for (l, r) in list.iter().zip(rightList.iter()) {
                                if let Some(cmp) = l.partial_cmp(r) {
                                    if cmp != Ordering::Equal {
                                        return Some(cmp);
                                    }
                                }
                            }
                            let rightSize = rightList.len();
                            return list.len().partial_cmp(&rightSize);
                        }
                    }
                }
            }
        } // self != right
    }
}


impl Default for Item
{
    fn default() -> Item
    {
        return Item::List(Default::default());
    }
}


fn parseList(line: &String, mut index: usize) -> (Item, usize)
{
    let mut buffer: Vec<char> = Default::default();
    let mut output: Item = Item::List(Default::default());

    loop {
        if let Some(c) = line.chars().skip(index).next() {
            index += 1;
            match c {
                '[' => {
                    let (parsed, consumed) = parseList(&line, index);
                    index += consumed - 1;
                    if let Item::List(out) = &mut output {
                        out.push(parsed);
                    }
                },
                ',' => {
                    if let Ok(integer) = buffer.iter().collect::<String>().parse::<i8>() {
                        if let Item::List(out) = &mut output {
                            out.push(Item::Int(integer));
                        }
                    }
                    buffer.clear();
                },
                ']' => {
                    if let Ok(integer) = buffer.iter().collect::<String>().parse::<i8>() {
                        if let Item::List(out) = &mut output {
                            out.push(Item::Int(integer));
                        }
                    }
                    break;
                },
                ' ' => {},
                _ => {
                    buffer.push(c);
                },
            }
        } else {
            break;
        }
    } // while true

    return (output, index);
}


fn main()
{
    let mut tmp: Item = Default::default();
    let mut output: usize = 0;

    if let Ok(file) = std::fs::File::open("input") {
        for (i_line, line) in io::BufReader::new(&file).lines().flatten().filter(|l| l != "").enumerate() {
            let item = parseList(&line, 1);
            if i_line % 2 == 1 {
                output += (i_line + 1) / 2 * (tmp < item.0) as usize;
            } else {
                tmp = item.0;
            }
        } // for line in file
    } // if file

    println!("{}", output);
}
