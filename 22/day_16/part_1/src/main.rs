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
    destinations: Vec<(NodeID,i64,i64)> // {id, distance, score)}
}


impl Graph
{
    // Return a map of node IDs and their distances + discounted scores
    fn getScoresFrom(&self, begin: NodeID, turn: i64) -> Vec<(NodeID, i64, i64)>
    {
        // Init
        let mut toVisit: HashSet<&NodeID> = HashSet::from([&begin]);
        let mut map: HashMap<&NodeID,(i64,i64)> = self.map.iter()
                                                          .map(|(id, _)| (id, (if id == &begin {0} else {i64::MAX}, 0)))
                                                          .collect();

        loop {
            if let Some(id) = toVisit.iter().cloned().next() {
                toVisit.remove(id);
                let node = &self.map[id];
                let current = map[&id];
                for edge in node.edges.iter() {
                    if let Some(n) = map.get_mut(&edge.destination) {
                        let distance = current.0 + edge.length;
                        if distance < n.0 {
                            n.0 = distance;
                            n.1 = (END - turn - distance) * self.map[&edge.destination].value;
                            toVisit.insert(&edge.destination);
                        }
                    }
                } // for edge in edges
            } else {
                break;
            }
        } // while true

        return map.iter()
                  .map(|(id, properties)| (id.clone().clone(), properties.0, properties.1))
                  .collect();
    }


    fn findMaximumScore(&self, begin: &NodeID) -> i64
    {
        let mut beginState = State {
            node: begin.clone(),
            turn: 0,
            score: 0,
            destinations: self.getScoresFrom(begin.clone(), 0)
        };
        beginState.destinations.retain(|(id, _, _)| id != begin);
        let mut states: Vec<State> = Vec::from([beginState.clone()]);
        let mut solution: i64 = 0;

        loop {
            match states.last_mut() {
                None => {
                    break; // <== states exhausted
                },
                Some(state) => {
                    match state.destinations.pop() {
                        None => {
                            states.pop(); // <== no nodes left to explore
                        },
                        Some((id, distance, score)) => {
                            let nextTurn = state.turn + distance;
                            if nextTurn < END {
                                let nextScore = state.score + score;
                                states.push(State {
                                    node: id.clone(),
                                    turn: nextTurn,
                                    score: nextScore,
                                    destinations: self.getScoresFrom(id.clone(), nextTurn)
                                        .into_iter()
                                        .filter(|(ID, _, _)| &id != ID && !states.iter().any(|s| &s.node == ID)) // <== filter visited nodes
                                        .collect()
                                }); // push state
                                solution = solution.max(nextScore);
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
                                                             .filter(|(_, _, score)| 0 < *score)
                                                             .map(|(key, distance, _)| Edge {
                                                                 destination: key.clone(),
                                                                 length: distance.clone()})
                                                             .collect()
                } // construct a Node
            ); // insert into tmp graph
        } // for id, node in graph
        graph = tmp;
    }

    println!("{}", graph.findMaximumScore(&begin));
}
