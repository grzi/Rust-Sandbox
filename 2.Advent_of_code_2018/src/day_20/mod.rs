use std::collections::{VecDeque, HashMap};

pub fn execute(_input: String) -> (isize, usize) {
    let mut grid = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    let mut dist : isize = 0;
    let mut stack = VecDeque::new();

    for character in (&_input[1.._input.len()-1]).chars().collect::<Vec<char>>(){
        match character {
            '(' =>{
                stack.push_back((dist,x,y));
            },
            ')' => {
                let popped = stack.pop_back().unwrap();
                dist = popped.0;
                x = popped.1;
                y = popped.2;
            },
            '|' => {
                let last = stack[stack.len()-1];
                dist = last.0;
                x = last.1;
                y = last.2;
            },
            _ => {
                x += (character == 'E') as isize - (character == 'W') as isize;
                y +=  (character == 'S') as isize - (character == 'N') as isize;
                dist += 1;
                if !grid.contains_key(&(x,y)) || dist < grid[&(x,y)]{
                    grid.entry((x,y)).and_modify(|e| *e = dist).or_insert(dist);
                }
            }
        }
    }
    (
        *grid.values().max().unwrap()
        ,
        grid.values().filter(|e| **e >= 1000).count()
        )
}
