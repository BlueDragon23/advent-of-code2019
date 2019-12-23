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
    let mut phases = vec![5,6,7,8,9];
    let output = phases.permutation().fold(0, |acc, permuted_phases| {
        let current_output = run_amplifier(xs.clone(), permuted_phases);
        max(acc, current_output)
    });
    println!("{}", output);
}

fn run_amplifier(instructions: Vec<i32>, phases: Vec<i32>) -> i32 {
    let mut states = vec![State { 
        pc: 0, 
        memory: instructions.clone(), 
        output: 0, 
        terminated: false, 
        changed_output: false, 
        has_used_phase: false 
    }; phases.len()];
    while !(states.last().unwrap().terminated) {
        states = phases.iter().zip(states.iter()).scan(states.last().unwrap().output, |output, (phase, state)| {
            // Pass the previous output to the next phase
            let result = run_program(*phase, state.clone(), *output);
            *output = result.output;
            Some(result)
        }).collect();
    }
    states.last().unwrap().output
}

fn run_program(phase: i32, mut state: State, prev_state_output: i32) -> State {
    loop {
        // println!("{:?}", xs);
        let (mode1, mode2, mode3, op) = parse_operation(state.memory[state.pc]);
        state = execute_operation(mode1, mode2, mode3, op, state.clone(), |has_used_phase: bool| {
            // If generators were stable, I'd do that
            if has_used_phase {
                prev_state_output
            } else {
                phase
            }
        });
        if state.terminated || state.changed_output {
            return state
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    pc: usize,
    memory: Vec<i32>,
    output: i32,
    terminated: bool,
    changed_output: bool,
    has_used_phase: bool,
}

fn execute_operation(mode1: u32, mode2: u32, mode3: u32, op: i32, state: State, mut get_input: impl FnMut(bool) -> i32) -> State {
    let pc = state.pc;
    let mut memory = state.memory;
    let output = state.output;
    let has_used_phase = state.has_used_phase;
    let param1 = if op != 99 { get_param(mode1, memory.clone(), pc, 1) } else { 0 };
    return match op {
        99 => State { pc, memory, output, terminated: true, changed_output: false, has_used_phase },
        1 => {
            // Add
            let param2 = get_param(mode2, memory.clone(), pc, 2);
            let target = memory[pc + 3];
            memory[target as usize] = param1 + param2;
            State { pc: pc + 4, memory, output, terminated: false, changed_output: false, has_used_phase }
        },
        2 => {
            // Multiply
            let param2 = get_param(mode2, memory.clone(), pc, 2);
            let target = memory[pc + 3];
            memory[target as usize] = param1 * param2;
            State { pc: pc + 4, memory, output, terminated: false, changed_output: false, has_used_phase }
        },
        3 => {
            // Input
            let input = get_input(has_used_phase);
            let target = memory[pc + 1];
            memory[target as usize] = input;
            State { pc: pc + 2, memory, output, terminated: false, changed_output: false, has_used_phase: true }
        },
        4 => {
            // Output
            State { pc: pc + 2, memory, output: param1, terminated: false, changed_output: true, has_used_phase }
        },
        5 => {
            // Jump if true
            let param2 = get_param(mode2, memory.clone(), pc, 2);
            if param1 != 0 {
                State { pc: param2 as usize, memory, output, terminated: false, changed_output: false, has_used_phase }
            } else {
                State { pc: pc + 3, memory, output, terminated: false, changed_output: false, has_used_phase }
            }
        },
        6 => {
            // Jump if false
            let param2 = get_param(mode2, memory.clone(), pc, 2);
            if param1 == 0 {
                State { pc: param2 as usize, memory, output, terminated: false, changed_output: false, has_used_phase }
            } else {
                State { pc: pc + 3, memory, output, terminated: false, changed_output: false, has_used_phase }
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
            State { pc: pc + 4, memory, output, terminated: false, changed_output: false, has_used_phase }
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
            State { pc: pc + 4, memory, output, terminated: false, changed_output: false, has_used_phase }
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