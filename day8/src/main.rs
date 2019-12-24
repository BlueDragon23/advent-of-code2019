use std::io::BufRead;
use std::fs::File;
use std::io::BufReader;
use std::str;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);
    let data = reader.lines().next().unwrap().unwrap();
    let layers: Vec<&[u8]> = get_layers(&data);
    let image = layers.iter().fold(vec!['2' as u8; WIDTH * HEIGHT], |acc, layer| {
        acc.iter().zip(layer.iter()).map(|(a, b)| {
            if *a == '2' as u8 {
                *b
            } else {
                *a
            }
        }).collect()
    });
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            print!("{}", if image[i * WIDTH + j] == '1' as u8 { '#' } else { '.' });
        }
        println!("");
    }
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