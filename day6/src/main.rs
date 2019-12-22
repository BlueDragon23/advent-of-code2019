use std::io::BufRead;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;
extern crate trees;
use trees::tr;

fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);
    // Create a map of node to children
    let map = reader
        .lines()
        .map(|line| line.unwrap())
        .fold(HashMap::<String, Vec<String>>::new(), add_node);
    // DFS to count. Start from COM
    let result = count_orbits(map, "COM".to_string(), 0);
    println!("{}", result)
}

fn add_node(mut map: HashMap::<String, Vec<String>>, unwrapped: String) -> HashMap::<String, Vec<String>> {
    let mut split = unwrapped.trim().split(")");
    let parent = split.next().unwrap().to_string();
    let child = split.next().unwrap().to_string();
    map
        .entry(parent.clone())
        .and_modify(|v| v.push(child.clone()))
        .or_insert(vec![child.clone()]);
    map
}

fn count_orbits(map: HashMap::<String, Vec<String>>, node: String, depth: u32) -> u32 {
    return map.get(&node).map_or(0, |children| children.iter().fold(0, |acc, child| {
        return acc + count_orbits(map.clone(), child.to_string(), depth + 1) + depth + 1;
    }));
}