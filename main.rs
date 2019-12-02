use std::io::BufRead;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashSet;

fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);
}