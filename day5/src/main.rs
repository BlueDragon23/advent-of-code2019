use std::io::BufRead;
use std::fs::File;
use std::io::BufReader;
use std::option::Option;

fn main() {
    let f = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(f);
    let mut buf = String::new();
    reader.read_line(&mut buf);
    let original_xs: Vec<i32> = buf
        .trim()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let mut xs = original_xs.clone();
    let mut pc = 0;
    loop {
        // println!("{:?}", xs);
        let (mode1, mode2, mode3, op) = parse_operation(xs[pc]);
        match execute_operation(mode1, mode2, mode3, op, xs, pc) {
            (None, new_xs) => {
                // xs = new_xs;
                break;
            },
            (Some(inc), new_xs) => {
                xs = new_xs;
                pc += inc;
            }
        }
    }
}

fn execute_operation(mode1: u32, mode2: u32, mode3: u32, op: i32, mut memory: Vec<i32>, pc: usize) -> (Option<usize>, Vec<i32>) {
    return match op {
        99 => (None, memory),
        1 => {
            let param1 = if mode1 == 1 { memory[pc + 1] } else { memory[memory[pc + 1] as usize] };
            let param2 = if mode2 == 1 { memory[pc + 2] } else { memory[memory[pc + 2] as usize] };
            let target = memory[pc + 3];
            memory[target as usize] = param1 + param2;
            return (Some(4), memory);
        },
        2 => {
            let param1 = if mode1 == 1 { memory[pc + 1] } else { memory[memory[pc + 1] as usize] };
            let param2 = if mode2 == 1 { memory[pc + 2] } else { memory[memory[pc + 2] as usize] };
            let target = memory[pc + 3];
            memory[target as usize] = param1 * param2;
            return (Some(4), memory);
        },
        3 => {
            // Input?
            let input = 1;
            let target = memory[pc + 1];
            memory[target as usize] = input;
            return (Some(2), memory);
        },
        4 => {
            // Output?
            let param1 = if mode1 == 1 { memory[pc + 1] } else { memory[memory[pc + 1] as usize] };
            println!("Output: {}, PC: {}", param1, pc);
            if param1 != 0 {
                println!("{:?}", memory);
            }
            return (Some(2), memory);
        },
        _ => {
            panic!("Unknown op {}", op);
        }
    }
}

// return (mode1, mode2, mode3, op)
fn parse_operation(command: i32) -> (u32, u32, u32, i32) {
    let command_str = format!("{:05}", command);
    let mut iter = command_str.chars();
    let mode3 = iter.next().unwrap().to_digit(10).unwrap();
    let mode2 = iter.next().unwrap().to_digit(10).unwrap();
    let mode1 = iter.next().unwrap().to_digit(10).unwrap();
    let mut op = String::new();
    op.push(iter.next().unwrap());
    op.push(iter.next().unwrap());
    // println!("{}{}{}{}", mode1, mode2, mode3, op);
    return (mode1, mode2, mode3, op.as_str().parse::<i32>().unwrap());
}