#![allow(non_snake_case)]

// --- STD Imports ---
use std::io::{self, BufRead};
use std::collections::{HashSet, HashMap};
use regex::Regex;


const END: i64 = 30;


type NodeID = String;


#[derive(Clone)]
struct Edge
{
    destination: NodeID,
    length: i64
}


#[derive(Clone)]
struct Node
{
    value: i64,
    edges: Vec<Edge>,
}


#[derive(Default)]
struct Graph
{
    map: HashMap<NodeID,Node>,
}


#[derive(Clone)]
struct State {
    node: NodeID,
    turn: i64,
    score: i64,
    destinations: HashMap<NodeID,(i64,i64)> // {id, (distance, score)}
}


impl Graph
{
    // Return a map of node IDs and their distances + discounted scores
    fn getScoresFrom(&self, begin: NodeID, turn: i64) -> HashMap<NodeID,(i64, i64)>
    {
        // Init
        let mut visited: HashSet<NodeID> = HashSet::from([begin.clone()]);
        let mut map: HashMap<NodeID,(i64,i64)> = Default::default();
        for id in self.map.keys() {
            if id == &begin {
                map.insert(id.clone(), (0, 0));
            } else {
                map.insert(id.clone(), (i64::MAX, 0));
            }
        }

        let mut toVisit: HashSet<NodeID> = HashSet::from([begin]);
        loop {
            if let Some(id) = toVisit.iter().next().cloned() {
                toVisit.remove(&id);
                visited.insert(id.clone());
                let node = &self.map[&id];
                let current = map[&id].clone();
                for edge in node.edges.iter() {
                    if let Some(n) = map.get_mut(&edge.destination) {
                        let distance = current.0 + edge.length;
                        if distance < n.0 {
                            n.0 = distance;
                            n.1 = (END - turn - distance) * self.map[&edge.destination].value;
                        }
                    }
                    if !visited.contains(&edge.destination) {
                        toVisit.insert(edge.destination.clone());
                    }
                } // for edge in edges
            } else {
                break;
            }
        } // while true

        return map;
    }


    fn findMaximumScore(&self, begin: &NodeID) -> Vec<State>
    {
        let mut beginState = State {
            node: begin.clone(),
            turn: 0,
            score: 0,
            destinations: self.getScoresFrom(begin.clone(), 0)
        };
        beginState.destinations.retain(|id, _| id != begin);
        let mut states: Vec<State> = Vec::from([beginState.clone()]);
        let mut solution: Vec<State> = states.clone();

        loop {
            match states.last().cloned() {
                None => {
                    break; // <== states exhausted
                },
                Some(state) => {
                    match state.destinations.iter().next() {
                        None => {
                            states.pop(); // <== no nodes left to explore
                        },
                        Some((id, (distance, score))) => {
                            let numberOfStates = states.len();
                            states[numberOfStates - 1].destinations.remove(id); // <== remove destination as visited
                            let nextTurn = state.turn + distance;
                            if nextTurn < END {
                                let nextScore = state.score + score;
                                states.push(State {
                                    node: id.clone(),
                                    turn: nextTurn,
                                    score: nextScore,
                                    destinations: self.getScoresFrom(id.clone(), nextTurn)
                                        .into_iter()
                                        .filter(|(ID, _)| id != ID && !states.iter().any(|s| &s.node == ID)) // <== filter visited nodes
                                        .collect()
                                }); // push state
                                if solution[solution.len() - 1].score < nextScore {
                                    solution = states.clone();
                                } // update the solution if the current path is better
                            } // if nextTurn < END
                        } // popped a destination node
                    } // try popping a destination node
                } // got latest state
            } // try grabbing the latest state
        } // loop

        return solution;
    }
} // impl Graph


fn main()
{
    let mut graph: Graph = Default::default();
    let begin = "AA".to_string();

    if let Ok(pattern) = Regex::new(r"Valve ([A-Z]+) has flow rate=([0-9]+); tunnels? leads? to valves? (.*)") {
        if let Ok(sinkPattern) = Regex::new(r"([A-Z]+)") {
            if let Ok(file) = std::fs::File::open("input") {
                for line in io::BufReader::new(&file).lines().flatten() {
                    if let Some(m) = pattern.captures(&line) {
                        let captures: Vec<String> = [m.get(1), m.get(2), m.get(3)].iter()
                                                                                  .flatten()
                                                                                  .map(|c| String::from(c.as_str()))
                                                                                  .collect();
                        if let (id, Ok(value), mut edges) = (
                                captures[0].clone(),
                                captures[1].parse::<i64>(),
                                sinkPattern.captures_iter(&captures[2])
                                           .map(|c| c.get(1))
                                           .flatten()
                                           .map(|c| String::from(c.as_str()))
                                           .collect::<Vec<String>>()) {
                            // Add a special node if the value is positive
                            if 0 < value {
                                let extraID = captures[0].clone() + "Z";
                                edges.push(extraID.clone());
                                graph.map.insert(
                                    extraID.clone(),
                                    Node {
                                        value: value,
                                        edges: Vec::from([Edge {destination: id.clone(), length: 0}])
                                    } // island node
                                ); // graph insert island node
                            } // if node with positive value
                            // Add the original node without its value
                            graph.map.insert(
                                id.clone(),
                                Node {
                                    value: 0,
                                    edges: edges.iter().map(|destination| Edge {destination: destination.clone(), length: 1}).collect(),
                                } // node
                            ); // graph.insert
                        } // strings => values
                    } // if pattern.match
                } // for line in file
            } // if file
        } // if sinkPattern
    } // if pattern

    // Reduce the graph into a fully connected one, in which
    // the only nodes left are the ones with non-zero values (+ begin),
    // and edge lengths represent the distances between them in the base graph.
    {
        let mut tmp: Graph = Default::default();
        for (id, node) in graph.map.iter().filter(|(id, node)| 0 < node.value || id == &&begin) {
            tmp.map.insert(
                id.clone(),
                Node {
                    value: node.value,
                    edges: graph.getScoresFrom(id.clone(), 0).iter()
                                                             .filter(|(_, properties)| 0 < properties.1)
                                                             .map(|(key, properties)| Edge {
                                                                 destination: key.clone(),
                                                                 length: properties.0})
                                                             .collect()
                } // construct a Node
            ); // insert into tmp graph
        } // for id, node in graph
        graph = tmp;
    }

    let solution = graph.findMaximumScore(&begin);
    println!("node\tturn\tscore");
    for state in solution.iter() {
        println!("{}\t{}\t{}", state.node, state.turn, state.score);
    }
}
