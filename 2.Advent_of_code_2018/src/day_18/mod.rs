use std::collections::HashMap;
pub fn execute(_input: String) -> (usize, usize) {
    (run (&_input,10),run (&_input,1000000000))
}

fn run(_input: &String, iter : usize) -> usize{
    let mut board : Vec<Vec<char>> = _input.lines().map(|l| l.chars().collect()).collect();

    let mut previous_iter: HashMap<Vec<Vec<char>>, usize> = HashMap::new();

    for iteration in 0..  {

        // condition d'arret part 1
        if iteration == iter{
            let mut register = HashMap::new();
            board.iter().for_each(|e| e.iter().for_each(|v| *register.entry(*v).or_insert(0) += 1));
            return register[&'#'] * register[&'|']
        }

        // condition d'arret part 2
        if previous_iter.contains_key(&board) {
            let cycle = iteration - previous_iter[&board];
            let delta = iteration - cycle;
            let final_cycle = (iter - delta) % cycle + delta;

            for (key, val) in previous_iter.iter() {
                if *val == final_cycle {
                    let mut register = HashMap::new();
                    key.iter().for_each(|e| e.iter().for_each(|v| *register.entry(*v).or_insert(0) += 1));
                    return register[&'#'] * register[&'|'];
                }
            }
        }

        previous_iter.insert(board.clone(),iteration);

        let mut future_board  = board.clone();

        for (line_nb, line) in board.iter().enumerate() {
            for (col_nb, cell) in line.iter().enumerate() {
                let mut neighbours = Vec::new();
                for delta_x in -1..=1{
                    for delta_y in -1..=1{
                        let dx : isize = line_nb as isize + delta_x;
                        let dy : isize = col_nb as isize + delta_y;
                        if dx >= 0 && dy >= 0 && dy < board.len() as isize && dx < board.len() as isize {
                            neighbours.push(board[dx as usize][dy as usize])
                        }
                    }
                }

                let mut register = HashMap::new();
                neighbours.iter().for_each(|e| *register.entry(*e).or_insert(0)+=1);

                match cell {
                    '.' => {
                        if register.contains_key(&'|') && register[&'|'] >= 3 {
                            future_board[line_nb][col_nb] = '|';
                        }
                    },
                    '|' => {
                        if register.contains_key(&'#') && register[&'#'] >= 3 {
                            future_board[line_nb][col_nb] = '#';
                        }
                    },
                    '#' => {
                        future_board[line_nb][col_nb] = {
                            if register.contains_key(&'#') && register[&'#'] >= 2 && register.contains_key(&'|') && register[&'|'] >= 1 {
                                '#'
                            }
                            else {
                                '.'
                            }
                        };
                    },
                    _ => {}
                }
            }
        }
        board = future_board;
    }
    0
}
