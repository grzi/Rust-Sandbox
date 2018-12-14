pub fn execute() -> (String, usize) {
    let input = 360781;
    let part1 = resolve(input, &part_1_condition);
    let part2 = resolve(input, &part_2_condition);
    (part1[input..(input + 10)].iter().map(|d| d.to_string()).collect::<String>(), part2.len() - 6)
}

fn resolve(_input: usize, condition: &Fn(usize, &Vec<usize>) -> bool) -> Vec<usize> {
    let mut recipes = vec!(3, 7);
    let mut elf_1 = 0;
    let mut elf_2 = 1;

    while condition(_input, &recipes) {
        let sum = recipes[elf_1] + recipes[elf_2];

        if sum < 10 {
            recipes.push(sum);
        } else {
            recipes.push(1);
            if !condition(_input, &recipes) { break; }
            recipes.push(sum % 10);
        }

        elf_1 = (elf_1 + 1 + recipes[elf_1]) % recipes.len();
        elf_2 = (elf_2 + 1 + recipes[elf_2]) % recipes.len();
    }
    recipes
}

fn part_1_condition(_input: usize, _recipes: &Vec<usize>) -> bool { _recipes.len() < _input + 10 }

fn part_2_condition(_input: usize, _recipes: &Vec<usize>) -> bool {
    _recipes.len() < _input.to_string().len()
        ||
        _recipes[_recipes.len() - _input.to_string().len()..].iter().map(|d| d.to_string()).collect::<String>() != _input.to_string()
}
