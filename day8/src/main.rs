use std::io::BufRead;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);
    let width = 25;
    let height = 6;
    let data = reader.lines().next().unwrap().unwrap();
    let layers: Vec<&[u8]> = data
        .trim()
        .as_bytes()
        .chunks(width * height)
        .collect();
    let layer_with_fewest_0 = layers
        .iter()
        .min_by_key(|layer| {
            layer.iter().fold(0, |count, c| {
                if *c == '0' as u8 {
                    return count + 1
                } else {
                    return count
                }
            })
        })
        .unwrap();
    let result = layer_with_fewest_0
        .iter()
        .filter(|x| **x == '1' as u8)
        .collect::<Vec<&u8>>()
        .len()
    * 
    layer_with_fewest_0
        .iter()
        .filter(|x| **x == '2' as u8)
        .collect::<Vec<&u8>>()
        .len();
    println!("{}", result);
}