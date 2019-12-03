use std::io::BufRead;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashSet;

fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);
    let sets: Vec<_> = reader.lines().map(|line| get_set_from_line(&line.unwrap())).collect();
    println!("{:?}", sets);
    for set in sets.iter() {
        println!("{}", set.len())
    }
    println!("{:?}", find_closest_point(sets));
}

fn find_closest_point(sets: Vec<HashSet::<(i32, i32)>>) -> (i32, i32) {
    let insersecting_points: HashSet::<_> = sets
        .iter()
        .fold(sets[0].clone(), |s1, s2| s1.intersection(s2).cloned().collect());
    let min_tuple = insersecting_points
        .iter()
        .fold((100_000i32, 100_000i32), |(x1, y1), (x2, y2)| {
            if x1.abs() + y1.abs() < x2.abs() + y2.abs() {
                return (x1, y1);
            } else {
                return (*x2, *y2);
            }
        });
    return min_tuple;
}

// Build a set of all positions a line passes through
fn get_set_from_line(line: &str) -> HashSet::<(i32, i32)> {
    let mut coords: HashSet::<(i32, i32)> = HashSet::new();
    let mut pos = (0, 0);
    let actions: Vec<&str> = line
        .trim()
        .split(',')
        .collect();
    for action in actions.iter() {
        let (direction, distance_str) = action.split_at(1);
        let distance = distance_str.parse::<i32>().unwrap();
        let (next_coords, next_pos) = do_move(coords.clone(), pos, direction, distance);
        pos = next_pos;
        coords = next_coords;
    }
    return coords;
}

fn do_move(mut coords: HashSet::<(i32, i32)>, current_point: (i32, i32), direction: &str, distance: i32) -> (HashSet::<(i32, i32)>, (i32, i32)) {
    let mut next_point = current_point;
    println!("Moving from {:?} {} {}", current_point, direction, distance);
    for _ in 0..distance {
        next_point = match direction {
            "U" => (next_point.0, next_point.1 + 1),
            "R" => (next_point.0 + 1, next_point.1),
            "D" => (next_point.0, next_point.1 - 1),
            "L" => (next_point.0 - 1, next_point.1),
            _ => {
                println!("Invalid direction");
                (next_point.0, next_point.1)
            }
        };
        coords.insert(next_point);
    }
    println!("Ended up at {:?}", next_point);
    return (coords, next_point);
}