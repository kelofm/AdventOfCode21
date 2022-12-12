#![allow(non_snake_case)]

// --- STD Imports ---
use std::io::{self, BufRead};
use std::collections::{HashSet, HashMap};
use std::collections::BinaryHeap;
use std::cmp::Ordering;


fn getNeighbours(position: usize, map: &Vec<u8>, size: usize) -> [Option<usize>;4]
{
    let cutoff = |i: i64| if 0 <= i && i < map.len() as i64 && (map[i as usize] as i64 - map[position] as i64) < 2 {Some(i as usize)} else {None};
    return [
        cutoff(position as i64 - 1),
        cutoff(position as i64 + 1),
        cutoff(position as i64 - size as i64),
        cutoff(position as i64 + size as i64)
    ];
}


struct Node
{
    neighbours: [Option<usize>; 4],
    distance: usize,
    predecessor: usize,
    visited: bool
}

struct Tile
{
    height: usize,
    position: usize
}


impl PartialEq for Tile
{
    fn eq(&self, right: &Tile) -> bool
    {
        return right.position == self.position;
    }
}


impl Eq for Tile {}


impl PartialOrd for Tile
{
    fn partial_cmp(&self, right: &Self) -> Option<Ordering>
    {
        return right.height.partial_cmp(&self.height);
    }
}


impl Ord for Tile
{
    fn cmp(&self, right: &Tile) -> Ordering
    {
        return right.height.cmp(&self.height);
    }
}


fn main()
{
    let mut map: Vec<u8> = Default::default();
    let mut queue: HashSet<usize> = Default::default();
    let mut toVisit: BinaryHeap<Tile> = Default::default();
    let mut graph: HashMap<usize,Node> = Default::default();

    let mut size: usize = 1;
    let mut begin: usize = 0;
    let mut end: usize = 0;

    // Read input
    if let Ok(file) = std::fs::File::open("input") {
        for (i_line, maybeLine) in io::BufReader::new(&file).lines().enumerate() {
            if let Ok(line) = maybeLine {
                for (i_c, c) in line.chars().enumerate() {
                    size = size.max(i_c + 1);
                    if c == 'S' {
                        map.push(0);
                        begin = i_line * size + i_c;
                    } else if c == 'E' {
                        map.push('z' as u8 - 'a' as u8);
                        end = i_line * size + i_c;
                    } else {
                        map.push(c as u8 - 'a' as u8);
                    }
                } // for char in line
            } // if line
        } // for maybeLine in file
    } // if file

    // Build graph
    for i_height in 0..map.len() {
        graph.insert(i_height, Node {
            neighbours: getNeighbours(i_height, &map, size),
            distance: usize::MAX,
            predecessor: usize::MAX,
            visited: false
        });
    } // for height in map

    // Init
    if let Some(beginNode) = graph.get_mut(&begin) {
        beginNode.distance = 0;
    }
    toVisit.push(Tile{height: 0, position: begin});

    // Find path
    loop {
        if let Some(position) = toVisit.pop() {
            queue.remove(&position.position);
            if position.position == end {
                break;
            }
            if let Some(node) = graph.get_mut(&position.position) {
                node.visited = true;
            }
            let neighbourDistance = graph[&position.position].distance + 1;
            let neighbours: Vec<usize> = graph[&position.position].neighbours.iter().flatten().copied().collect();
            for neighbour in neighbours {
                if let Some(node) = graph.get_mut(&neighbour) {
                    if neighbourDistance < node.distance {
                        node.distance = neighbourDistance;
                        node.predecessor = position.position;
                    }
                    if !node.visited && !queue.contains(&neighbour) {
                        toVisit.push(Tile{height: node.distance, position: neighbour});
                        queue.insert(neighbour);
                    }
                }
            } // for neighbour in neighbours
        } else {
            break;
        } // if toVisit.empty()
    } // while 1

    println!("{}", graph[&end].distance);
}
