use std::collections::{HashSet, VecDeque, HashMap};
use std::cmp::Ordering;
use std::char;

pub fn execute(_input: String) {
    println!("step 1 : {:?}", run(&_input, 3).0);
    let mut attaq = 4;
    for attaq in 4..{
        let result = run(&_input,attaq);
        let mut dead_elf = false;
        for key in result.1.keys(){
            if is_elf(*key) && result.1[key] <= 0{
                dead_elf = true;
                break;
            }
        }
        if !dead_elf {
            println!("step 2 : {} ", result.0);
            break;
        }
    }
}

fn run(_input: &String, _gob_att: isize) -> (isize, HashMap<char,isize>) {
    let mut board = _input.lines().map(|e| e.trim().chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut e_size = 0;
    let mut g_size = 0;
    let mut hp: HashMap<char, isize> = HashMap::new();
    let mut turn = 0;

    for i in 0..board.len() {
        for j in 0..board[i].len() {
            if board[i][j] == 'G' {
                board[i][j] = char::from_u32(65 + g_size).unwrap();
                hp.insert(board[i][j], 200);
                g_size += 1;
            } else if board[i][j] == 'E' {
                board[i][j] = char::from_u32(97 + e_size).unwrap();
                hp.insert(board[i][j], 200);
                e_size += 1;
            }
        }
    }

    loop {
        let players = get_players(&board);
        for _hero in players.iter() {
            let mut hero = (_hero.0, _hero.1, _hero.2);
            if hp[&hero.2] <= 0 {
                continue;
            }

            let next = compute(&hero, &board);

            match next {
                Some(val) => {
                    if val != (hero.0, hero.1) {
                        // move
                        board[hero.0][hero.1] = '.';
                        hero.0 = val.0;
                        hero.1 = val.1;
                        board[val.0][val.1] = hero.2
                    }
                }
                None => {
                    let tmp_players = get_players(&board);
                    let gobelins: Vec<(usize, usize, char)> = tmp_players.iter().filter(|e| is_gob(e.2)).cloned().collect();
                    if (tmp_players.len() - gobelins.len()) * gobelins.len() == 0 {
                        // end
                        let hps = hp.values().map(|e| *e).collect::<Vec<isize>>();
                        let hps = hps.iter().filter(|e| *e > &0).cloned().collect::<Vec<isize>>();
                        let sum: isize = hps.iter().sum();
                        return (sum * turn, hp);
                    }
                }
            }

            let mut min = (1000, 0, 0);
            for delta in vec![(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
                let dx = (hero.0 as i32 - delta.0) as usize;
                let dy = (hero.1 as i32 - delta.1) as usize;

                if is_gob_or_elf(board[dx][dy]) && is_elf(board[dx][dy]) != is_elf(hero.2) {
                    min = min_three_dimension(&min, &(hp[&board[dx][dy]] as usize, dx, dy));
                }
            }

            if min == (1000, 0, 0) {
                continue;
            }

            let enemy = board[min.1][min.2];
            let power = {
                if is_gob(enemy) { _gob_att } else { 3 }
            };

            *hp.entry(enemy).or_insert(200) -= power;
            if hp[&enemy] <= 0 {
                board[min.1][min.2] = '.';
            }
        }
        turn += 1;
    }
}

fn compute(_hero: &(usize, usize, char), _board: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    let targets = get_targets(_hero, _board);
    if targets.len() == 0 {
        return None;
    }
    let min_target = *targets.iter().min_by(|coord_1, coord_2| min_two_dimension(**coord_1, **coord_2)).unwrap();

    if min_target == (_hero.0, _hero.1) {
        return Some((_hero.0, _hero.1));
    }

    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();

    queue.push_front(min_target);
    visited.insert(min_target, 0);
    while queue.len() > 0 {
        let mut tmp_queue = VecDeque::new();

        for coord in queue.iter() {
            for delta in vec![(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
                let dx = (coord.0 as i32 - delta.0) as usize;
                let dy = (coord.1 as i32 - delta.1) as usize;

                visited.entry((dx, dy)).or_insert(10000);

                if visited[&(dx, dy)] != 10000 || _board[dx][dy] == '#' {
                    continue;
                }

                if _board[dx][dy] == '.' {
                    tmp_queue.push_back((dx, dy));
                    let val = visited[&coord];
                    *visited.get_mut(&(dx, dy)).unwrap() = val + 1;
                }
            }
        }
        queue = tmp_queue
    }

    let mut min = (10000, 0, 0);

    for delta in vec![(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
        let dx = (_hero.0 as i32 - delta.0) as usize;
        let dy = (_hero.1 as i32 - delta.1) as usize;
        visited.entry((dx, dy)).or_insert(10000);
        min = min_three_dimension(&min, &(visited[&(dx, dy)], dx, dy));
    }
    Some((min.1, min.2))
}

fn min_three_dimension(x: &(usize, usize, usize), y: &(usize, usize, usize)) -> (usize, usize, usize) {
    let x_val = x.0 * 10000 + x.1 * 1000 + x.2;
    let y_val = y.0 * 10000 + y.1 * 1000 + y.2;

    if x_val < y_val {
        return (x.0, x.1, x.2);
    }
    (y.0, y.1, y.2)
}

fn min_two_dimension(x: (usize, usize), y: (usize, usize)) -> Ordering {
    (x.0 * 100000 + x.1).cmp(&(y.0 * 100000 + y.1))
}

fn get_targets(_hero: &(usize, usize, char), _board: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_front((_hero.0, _hero.1));

    while queue.len() > 0 {
        let mut tmp_queue = VecDeque::new();
        let mut targets = Vec::new();
        for coord in queue.iter() {
            for delta in vec![(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
                let dx = (coord.0 as i32 - delta.0) as usize;
                let dy = (coord.1 as i32 - delta.1) as usize;
                if is_gob_or_elf(_board[dx][dy]) && is_elf(_board[dx][dy]) != is_elf(_hero.2) {
                    targets.push((coord.0, coord.1));
                }
                if visited.contains(&(dx, dy)) || _board[dx][dy] == '#' {
                    continue;
                }
                if _board[dx][dy] == '.' {
                    tmp_queue.push_back((dx, dy));
                    visited.insert((dx, dy));
                }
            }
        }
        if targets.len() > 0 {
            return targets;
        }
        queue = tmp_queue;
    }
    Vec::new()
}

fn get_players(_board: &Vec<Vec<char>>) -> Vec<(usize, usize, char)> {
    let mut pos = Vec::new();
    for i in 0.._board.len() {
        for j in 0.._board[i].len() {
            if is_elf(_board[i][j]) || is_gob(_board[i][j]) {
                pos.push((i, j, _board[i][j]));
            }
        }
    }
    pos
}

fn is_elf(c: char) -> bool {
    97 <= c as u32 && c as u32 <= 122
}

fn is_gob(c: char) -> bool {
    65 <= c as u32 && c as u32 <= 90
}

fn is_gob_or_elf(c: char) -> bool {
    is_gob(c) || is_elf(c)
}
