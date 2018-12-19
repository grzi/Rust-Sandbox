use day_16;

pub fn execute(_input: String) -> (usize, usize) {
    (run(&_input,0), run(&_input,1))
}

fn run (_input: &String, initial_val : usize) -> usize{
    let lines: Vec<&str> = _input.lines().collect();

    let functions = day_16::get_functions();
    let mut register = vec![0; 6];
    let mut instructions = Vec::new();
    let mut ip = 0;
    register[0] = initial_val;

    for line in lines {
        let split_line: Vec<&str> = line.trim().split(" ").collect();
        if split_line[0] == "#ip" {
            ip = split_line[1].parse::<usize>().unwrap();
            register[ip] = 0;
        }else{
            instructions.push(Instruction{
                opcode : split_line[0].to_string(),
                a : split_line[1].parse().unwrap(),
                b : split_line[2].parse().unwrap(),
                c : split_line[3].parse().unwrap(),
            });
        }
    }

    let mut position = 0;

    for _ in 0..100 {
        let instruction = &instructions[position];
        register[ip] = position;
        register[instruction.c] = functions[&instruction.opcode](instruction.a, instruction.b, &register);
        position = register[ip];
        position += 1;
    }

    let facto_to_calculate = register[2];

    let mut sum = 0;

    for i in 1..=facto_to_calculate{
        if facto_to_calculate % i == 0 {
            sum += i;
        }
    }
    sum
}

#[derive(Debug)]
struct Instruction {
    opcode: String,
    a: usize,
    b: usize,
    c: usize
}
