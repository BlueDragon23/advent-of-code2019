use std::io::BufRead;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let f = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(f);
    let mut buf = String::new();
    reader.read_line(&mut buf);
    let mut original_xs: Vec<i32> = buf
        .trim()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let goal_output = 19690720;
    let mut noun = 0;
    let mut verb = 0;
    loop {
        let mut xs = original_xs.clone();
        let mut i = 0;
        xs[1] = noun;
        xs[2] = verb;
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
        println!("{},{},{}", xs[0], noun, verb);
        if xs[0] == goal_output {
            break;
        }
        if verb < 99 {
            verb += 1;
        } else {
            noun += 1;
            verb = 0;
        }
    }
    println!("{}, {}", noun, verb);
}