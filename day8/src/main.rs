use std::io::BufRead;
use std::fs::File;
use std::io::BufReader;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);
    let data = reader.lines().next().unwrap().unwrap();
    let layers: Vec<&[u8]> = get_layers(&data);
    let layer_with_fewest_0 = layers
        .iter()
        .min_by_key(|layer| get_char_count(layer, '0'))
        .unwrap();
    let result = get_char_count(layer_with_fewest_0, '1') * get_char_count(layer_with_fewest_0, '2');
    println!("{}", result);
}

fn get_layers<'a>(data: &'a String) -> Vec<&'a [u8]> {
    data
        .trim()
        .as_bytes()
        .chunks(WIDTH * HEIGHT)
        .collect()
}

fn get_char_count(list: &&[u8], to_find: char) -> usize {
    list
        .iter()
        .filter(|x| **x == to_find as u8)
        .collect::<Vec<&u8>>()
        .len()
}