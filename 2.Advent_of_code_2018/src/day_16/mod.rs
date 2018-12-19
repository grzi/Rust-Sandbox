use std::collections::{HashMap, HashSet};

pub fn execute(_input: String) -> (usize, usize) {
    (part_1(&_input), part_2(&_input))
}

fn part_1(_input: &String) -> usize {
    let instructions = read_input(_input).0;
    let mut more_than_3 = 0;

    for instruction in instructions {
        let mut correct = 0;
        for function in get_functions().values() {
            let mut tmp_register = instruction.0.clone();
            tmp_register[instruction.1[3]] = function(instruction.1[1], instruction.1[2], &instruction.0);
            correct += (tmp_register == instruction.2) as usize;
        }
        more_than_3 += (correct >= 3) as usize;
    }
    more_than_3
}

fn part_2(_input: &String) -> usize {
    let inputs = read_input(_input);
    let functions = find_opcodes(&inputs.0);
    let mut register: Vec<usize> = vec![0; 4];

    for command in inputs.1 {
        register[command[3]] = functions[&command[0]](command[1], command[2], &register);
    }
    register[0]
}

fn find_opcodes(_instructions: &Vec<(Vec<usize>, Vec<usize>, Vec<usize>)>) -> HashMap<usize, fn(usize, usize, &Vec<usize>) -> usize> {
    let mut mapping = HashMap::new();
    let functions = get_functions();
    for function in functions.clone() {
        let mut compatible: HashSet<usize> = (0..16).collect();
        for instruction in _instructions.iter() {
            if !compatible.contains(&(instruction.1)[0]) {
                continue;
            }

            let mut tmp_register = instruction.0.clone();
            tmp_register[instruction.1[3]] = function.1(instruction.1[1], instruction.1[2], &instruction.0);

            if tmp_register != instruction.2 {
                compatible.remove(&instruction.1[0]);
            }
        }
        mapping.insert(function.0, compatible);
    }

    let mut final_mapping = HashMap::new();

    while final_mapping.len() < 16 {
        for function in mapping.iter() {
            let not_assigned = function.1.iter().filter(|e| !final_mapping.contains_key(*e)).collect::<Vec<&usize>>();

            if not_assigned.len() == 1 {
                final_mapping.insert(*not_assigned[0], functions[function.0]);
            }
        }
    }
    final_mapping
}

fn read_input(_input: &String) -> (Vec<(Vec<usize>, Vec<usize>, Vec<usize>)>, Vec<Vec<usize>>) {
    let lines = _input.lines().collect::<Vec<&str>>();
    let mut line = 0;
    let mut examples = Vec::new();
    let mut real_inputs = Vec::new();

    while line < lines.len() {
        if lines[line].is_empty() { line += 1; } else if lines[line].starts_with("Before") {
            let before = (&(lines[line])[9..lines[line].len() - 1]).split(",").collect::<Vec<&str>>().iter().map(|e| e.trim().parse::<usize>().unwrap()).collect::<Vec<usize>>();
            let command = lines[line + 1].split(" ").collect::<Vec<&str>>().iter().map(|e| e.trim().parse::<usize>().unwrap()).collect::<Vec<usize>>();
            let after = (&(lines[line + 2])[9..lines[line + 2].len() - 1]).split(",").collect::<Vec<&str>>().iter().map(|e| e.trim().parse::<usize>().unwrap()).collect::<Vec<usize>>();
            examples.push((before, command, after));
            line += 3;
        } else {
            let command = lines[line].split(" ").collect::<Vec<&str>>().iter().map(|e| e.trim().parse::<usize>().unwrap()).collect::<Vec<usize>>();
            real_inputs.push(command);
            line += 1;
        }
    }

    (examples, real_inputs)
}

pub fn get_functions() -> HashMap<String, fn(usize, usize, &Vec<usize>) -> usize> {
    let mut functions: HashMap<String, fn(usize, usize, &Vec<usize>) -> usize> = HashMap::new();
    functions.insert(String::from("addr"), |a, b, register| register[a] + register[b]);
    functions.insert(String::from("addi"), |a, b, register| register[a] + b);
    functions.insert(String::from("mulr"), |a, b, register| register[a] * register[b]);
    functions.insert(String::from("muli"), |a, b, register| register[a] * b);
    functions.insert(String::from("banr"), |a, b, register| register[a] & register[b]);
    functions.insert(String::from("bani"), |a, b, register| register[a] & b);
    functions.insert(String::from("borr"), |a, b, register| register[a] | register[b]);
    functions.insert(String::from("bori"), |a, b, register| register[a] | b);
    functions.insert(String::from("setr"), |a, _, register| register[a]);
    functions.insert(String::from("seti"), |a, _, _| a);
    functions.insert(String::from("gtir"), |a, b, register| (a > register[b]) as usize);
    functions.insert(String::from("gtri"), |a, b, register| (register[a] > b) as usize);
    functions.insert(String::from("gtrr"), |a, b, register| (register[a] > register[b]) as usize);
    functions.insert(String::from("eqir"), |a, b, register| (a == register[b]) as usize);
    functions.insert(String::from("eqri"), |a, b, register| (register[a] == b) as usize);
    functions.insert(String::from("eqrr"), |a, b, register| (register[a] == register[b]) as usize);
    functions
}
