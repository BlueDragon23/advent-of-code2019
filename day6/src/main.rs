use std::io::BufRead;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::result::Result;
extern crate trees;

fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);
    // Create a map of node to children
    let map = reader
        .lines()
        .map(|line| line.unwrap())
        .fold(HashMap::<String, Vec<String>>::new(), add_node);
    // DFS to count. Start from COM
    // let result = count_orbits(map, "COM".to_string(), 0);
    let result = orbital_distance(map);
    println!("{:?}", result)
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

fn orbital_distance(map: HashMap::<String, Vec<String>>) -> Result<usize, ()> {
    let mut you = find_path(map.clone(), "COM".to_string(), "YOU".to_string())?;
    let mut santa = find_path(map.clone(), "COM".to_string(), "SAN".to_string())?;
    // Strip common prefix
    while you[0] == santa[0] {
        you.pop_front();
        santa.pop_front();
    }
    Ok(you.len() + santa.len())
}

/**
 * Find the series of nodes leading to the target
 */
fn find_path(map: HashMap::<String, Vec<String>>, current: String, target: String) -> Result<VecDeque<String>, ()> {
    if current == target {
        let mut vec_deq = VecDeque::new();
        vec_deq.push_front(current);
        return Ok(vec_deq);
    }
    return map.get(&current).map_or(Err(()), |children| children.iter().fold(Err(()), |acc, child| {
        match acc {
            // We've already found the target
            Ok(path) => Ok(path),
            // Try to find the target
            Err(_e) => {
                match find_path(map.clone(), child.to_string(), target.clone()) {
                    Ok(mut path) => {
                        path.push_front(current.clone());
                        Ok(path)
                    },
                    Err(_err) => Err(_err),
                }
            },
        }
    }))
}

fn count_orbits(map: HashMap::<String, Vec<String>>, node: String, depth: u32) -> u32 {
    return map.get(&node).map_or(0, |children| children.iter().fold(0, |acc, child| {
        return acc + count_orbits(map.clone(), child.to_string(), depth + 1) + depth + 1;
    }));
}