use std::collections::{HashMap, HashSet };

pub fn execute(_input: String) -> (usize,usize) {
    (part_1(&_input), part_2(&_input)[0])
}

fn part_1(_input: &String) -> usize{
    let instructions = read_input(_input).0;
    let functions = get_functions();
    let mut more_than_3 = 0;

    for instruction in instructions{
        let mut correct = 0;
        for function in functions.values(){
            let mut res = instruction.0.clone();
            res[instruction.1[3]] = function(instruction.1[1],instruction.1[2], &instruction.0);
            if  res == instruction.2 {
                correct += 1;
            }
        }
        if correct >= 3 {
            more_than_3 +=1;
        }
    }

    more_than_3
}

fn part_2(_input : &String) -> Vec<usize>{
    let instructions_tests = read_input(_input);
    let functions = find_opcodes(&instructions_tests.0);
    let mut register : Vec<usize> = vec![0;4];

    for command in instructions_tests.1{
        let func = functions[&command[0]];
        register[command[3]] = func(command[1], command[2], &register);
    }
    register
}

fn find_opcodes(_instructions : &Vec<(Vec<usize>, Vec<usize>, Vec<usize>)>) -> HashMap<usize,fn(usize, usize, &Vec<usize>) -> usize>{
    let mut mapping = HashMap::new();
    let functions = get_functions();
    for function in functions.clone(){
        let mut compatible : HashSet<usize> = (0..16).collect();
        for instruction in _instructions.iter(){
            if !compatible.contains(&(instruction.1)[0]) {
                continue;
            }

            let mut res = instruction.0.clone();
            res[instruction.1[3]] = function.1(instruction.1[1],instruction.1[2], &instruction.0);

            if  res != instruction.2 {

                compatible.remove(&instruction.1[0]);
            }

        }
        mapping.insert(function.0, compatible);
    }

    let mut final_mapping = HashMap::new();

    while final_mapping.len() < 16 {
        for function in mapping.clone() {
            let not_assigned = function.1.iter().filter(|e| !final_mapping.contains_key(*e)).collect::<Vec<&usize>>();

            if not_assigned.len() == 1 {
                final_mapping.insert(*not_assigned[0], functions[&function.0]);
            }
        }
    }

    final_mapping
}

fn read_input(_input : &String) -> (Vec<(Vec<usize>, Vec<usize>, Vec<usize>)>, Vec<Vec<usize>>){
    let lines = _input.lines().collect::<Vec<&str>>();
    let mut line = 0;
    let mut instructions = Vec::new();
    let mut tests = Vec::new();

    while line < lines.len() {
        if lines[line].is_empty() { line+=1; }
        else if lines[line].starts_with("Before") {
            let before = (&(lines[line])[9..lines[line].len()-1]).split(",").collect::<Vec<&str>>().iter().map(|e| e.trim().parse::<usize>().unwrap()).collect::<Vec<usize>>();
            let command = lines[line+1].split(" ").collect::<Vec<&str>>().iter().map(|e| e.trim().parse::<usize>().unwrap()).collect::<Vec<usize>>();
            let after = (&(lines[line+2])[9..lines[line+2].len()-1]).split(",").collect::<Vec<&str>>().iter().map(|e| e.trim().parse::<usize>().unwrap()).collect::<Vec<usize>>();
            instructions.push((before, command, after));
            line+=3;
        }else{
            let command = lines[line].split(" ").collect::<Vec<&str>>().iter().map(|e| e.trim().parse::<usize>().unwrap()).collect::<Vec<usize>>();
            tests.push(command);
            line+=1;
        }
    }

    (instructions, tests)
}

fn get_functions() -> HashMap<String,fn(usize, usize, &Vec<usize>) -> usize>{
    let mut functions : HashMap<String,fn(usize,usize, &Vec<usize>) -> usize> = HashMap::new();
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
    functions.insert(String::from("gtir"), |a, b, register| { if a > register[b] { return 1 } 0 });
    functions.insert(String::from("gtri"), |a, b, register| {if register[a] > b { return 1 } 0});
    functions.insert(String::from("gtrr"), |a, b, register| {if register[a] > register[b] { return 1 } 0});
    functions.insert(String::from("egir"), |a, b, register| {if a == register[b] { return 1 } 0});
    functions.insert(String::from("egri"), |a, b, register| {if register[a] == b { return 1 } 0});
    functions.insert(String::from("egrr"), |a, b, register| {if register[a] == register[b] { return 1 } 0});
    functions
}
