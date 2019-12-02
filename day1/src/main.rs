use std::io::BufRead;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);
    let fuel_mass = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|x| x.parse::<i32>().unwrap())
        .map(|x| x/3 - 2)
        .map(|fuel| {
            let mut total = 0;
            let mut i = fuel;
            while i > 0 {
                total += i;
                i = i / 3 - 2;
            }
            return total;
        })
        .collect::<Vec<i32>>();
    
    println!("{}", fuel_mass.iter().sum::<i32>());
}