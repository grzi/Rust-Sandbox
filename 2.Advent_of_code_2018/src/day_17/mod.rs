use std::collections::VecDeque;

pub fn execute(_input: String) -> (usize, usize) {
    let data = read_input(_input);

    let min_y = data.iter().map(|e| e.0).min().unwrap() - 1;
    let min_x = data.iter().map(|e| e.1).min().unwrap();
    let max_y = data.iter().map(|e| e.0).max().unwrap() + 1;
    let max_x = data.iter().map(|e| e.1).max().unwrap();


    let mut grid = vec![vec!['.'; max_y - min_y + 1]; max_x + 1];

    grid[0][500 - min_y] = '+';

    for line in data.iter() {
        grid[line.1][line.0 - min_y] = '#';
    }
    let mut queue = VecDeque::new();
    queue.push_back((1, 500));

    while queue.len() > 0 {
        let current = queue.pop_front().unwrap();
        if grid[current.0][current.1 - min_y] == '.' {
            grid[current.0][current.1 - min_y] = '|';
        }

        if current.0 == max_x{
            continue;
        }

        if grid[current.0 + 1][current.1 - min_y] == '.' {
            queue.push_back((current.0 + 1, current.1));
            continue;
        } else if grid[current.0 + 1][current.1 - min_y] == '~' || grid[current.0 + 1][current.1 - min_y] == '#' {
            if grid[current.0][current.1 - min_y + 1] == '.' {
                queue.push_back((current.0, current.1 + 1));
            }
            if grid[current.0][current.1 - min_y - 1] == '.' {
                queue.push_back((current.0, current.1 - 1));
            }

            if (grid[current.0][current.1 - min_y + 1] == '|' || grid[current.0][current.1 - min_y + 1] == '#')
                && (grid[current.0][current.1 - min_y - 1] == '|' || grid[current.0][current.1 - min_y - 1] == '#') {

                let mut tmp = current.1;

                while grid[current.0][tmp-min_y+1] == '|' || grid[current.0][tmp-min_y+1] == '~' {
                    tmp += 1;
                }

                if grid[current.0][tmp-min_y+1] != '#'{
                    continue;
                }

                tmp = current.1;
                while grid[current.0][tmp-min_y-1] == '|' || grid[current.0][tmp-min_y-1] == '~' {
                    tmp -= 1;
                }

                if grid[current.0][tmp-min_y-1] != '#'{
                    continue;
                }

                tmp = current.1;
                grid[current.0][tmp-min_y] = '~';

                if grid[current.0-1][tmp-min_y] == '|' {
                    queue.push_back((current.0 - 1, tmp));
                }

                while grid[current.0][tmp-min_y+1] == '|' || grid[current.0][tmp-min_y+1] == '~' {
                    grid[current.0][tmp - min_y + 1] = '~';
                    tmp += 1;
                    if grid[current.0 - 1][tmp - min_y] == '|' {
                        queue.push_back((current.0 - 1, tmp));
                    }
                }

                while grid[current.0][tmp-min_y-1] == '|' || grid[current.0][tmp-min_y-1] == '~' {
                    grid[current.0][tmp - min_y - 1] = '~';
                    tmp -= 1;
                    if grid[current.0 - 1][tmp - min_y] == '|' {
                        queue.push_back((current.0 - 1, tmp));
                    }
                }
            }
        }
    }

    let mut tilde = 0;
    let mut water = 0;


    for i in min_x..=max_x {
        for j in 0..grid[i].len() {
            if grid[i][j] == '~' {
                tilde += 1;
            }
            if grid[i][j] == '|' {
                water += 1;
            }
        }
    }
    (tilde+water, tilde)
}

fn read_input(_input: String) -> Vec<(usize, usize)> {
    let mut data = Vec::new();
    for line in _input.lines().collect::<Vec<&str>>() {
        let values = line.split(",").collect::<Vec<&str>>();
        let field = values[0].split("=").collect::<Vec<&str>>()[1].trim().parse::<usize>().expect("Should be a positive number");
        let range = values[1].split("..").collect::<Vec<&str>>();
        let range_start = range[0].split("=").collect::<Vec<&str>>()[1].trim().parse::<usize>().expect("Should be a positive number");
        let range_end = range[1].trim().parse::<usize>().expect("Should be a positive number");

        if line.starts_with("x") {
            (range_start..=range_end).for_each(|y| data.push((field, y)));
        } else if line.starts_with("y") {
            (range_start..=range_end).for_each(|x| data.push((x, field)));
        }
    }
    data
}
