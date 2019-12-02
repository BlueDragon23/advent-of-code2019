use std::io::BufRead;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let f = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(f);
    let mut buf = String::new();
    reader.read_line(&mut buf);
    let mut xs: Vec<i32> = buf
        .trim()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    xs[1] = 12;
    xs[2] = 2;
    let mut i = 0;
    loop {
        // println!("{:?}", xs);
        let op = xs[i];
        let target = xs[i + 3];
        if op == 99 {
            break;
        } else if op == 1 {
            xs[target as usize] = xs[xs[i + 1] as usize] + xs[xs[i + 2] as usize];
        } else if op == 2 {
            xs[target as usize] = xs[xs[i + 1] as usize] * xs[xs[i + 2] as usize];
        }
        i += 4;
    }
    println!("{}", xs[0]);
}