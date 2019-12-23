use std::cmp::max;
use std::io::BufRead;
use std::fs::File;
use std::io::BufReader;

extern crate permutator;
use permutator::Permutation;

fn main() {
    let f = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(f);
    let mut buf = String::new();
    reader.read_line(&mut buf);
    let xs: Vec<i32> = buf
        .trim()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let mut phases = vec![0,1,2,3,4];
    let output = phases.permutation().fold(0, |acc, permuted_phases| {
        let current_output = run_amplifier(xs.clone(), permuted_phases);
        max(acc, current_output)
    });
    println!("{}", output);
}

fn run_amplifier(instructions: Vec<i32>, phases: Vec<i32>) -> i32 {
    phases.iter().fold(0, |acc, phase| {
        // Pass the previous output to the next phase
        run_program(instructions.clone(), *phase, acc)
    })
}

fn run_program(instructions: Vec<i32>, phase: i32, input: i32) -> i32 {
    let mut state = State { pc: 0, memory: instructions.clone(), output: 0, terminated: false };
    let mut gotten_input = false;
    loop {
        // println!("{:?}", xs);
        let (mode1, mode2, mode3, op) = parse_operation(state.memory[state.pc]);
        state = execute_operation(mode1, mode2, mode3, op, state.clone(), || {
            // If generators were stable, I'd do that
            if gotten_input {
                input
            } else {
                gotten_input = true;
                phase
            }
        });
        if state.terminated {
            return state.output
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    pc: usize,
    memory: Vec<i32>,
    output: i32,
    terminated: bool,
}

fn execute_operation(mode1: u32, mode2: u32, mode3: u32, op: i32, state: State, mut get_input: impl FnMut() -> i32) -> State {
    let pc = state.pc;
    let mut memory = state.memory;
    let output = state.output;
    let param1 = if op != 99 { get_param(mode1, memory.clone(), pc, 1) } else { 0 };
    return match op {
        99 => State { pc, memory, output, terminated: true },
        1 => {
            // Add
            let param2 = get_param(mode2, memory.clone(), pc, 2);
            let target = memory[pc + 3];
            memory[target as usize] = param1 + param2;
            State { pc: pc + 4, memory, output, terminated: false }
        },
        2 => {
            // Multiply
            let param2 = get_param(mode2, memory.clone(), pc, 2);
            let target = memory[pc + 3];
            memory[target as usize] = param1 * param2;
            State { pc: pc + 4, memory, output, terminated: false }
        },
        3 => {
            // Input
            let input = get_input();
            let target = memory[pc + 1];
            memory[target as usize] = input;
            State { pc: pc + 2, memory, output, terminated: false }
        },
        4 => {
            // Output
            println!("Output: {}, PC: {}", param1, pc);
            State { pc: pc + 2, memory, output: param1, terminated: false }
        },
        5 => {
            // Jump if true
            let param2 = get_param(mode2, memory.clone(), pc, 2);
            if param1 != 0 {
                State { pc: param2 as usize, memory, output, terminated: false }
            } else {
                State { pc: pc + 3, memory, output, terminated: false }
            }
        },
        6 => {
            // Jump if false
            let param2 = get_param(mode2, memory.clone(), pc, 2);
            if param1 == 0 {
                State { pc: param2 as usize, memory, output, terminated: false }
            } else {
                State { pc: pc + 3, memory, output, terminated: false }
            }
        },
        7 => {
            // Less than
            let param2 = get_param(mode2, memory.clone(), pc, 2);
            let target = memory[pc + 3] as usize;
            if param1 < param2 {
                memory[target] = 1;
            } else {
                memory[target] = 0;
            }
            State { pc: pc + 4, memory, output, terminated: false }
        },
        8 => {
            // Equals
            let param2 = get_param(mode2, memory.clone(), pc, 2);
            let target = memory[pc + 3] as usize;
            if param1 == param2 {
                memory[target] = 1;
            } else {
                memory[target] = 0;
            }
            State { pc: pc + 4, memory, output, terminated: false }
        },
        _ => {
            panic!("Unknown op {}", op);
        }
    }
}

fn get_param(mode: u32, memory: Vec<i32>, pc: usize, offset: usize) -> i32 {
    return if mode == 1 { memory[pc + offset] } else { memory[memory[pc + offset] as usize] };
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