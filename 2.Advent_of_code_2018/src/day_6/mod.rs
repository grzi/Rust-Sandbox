use std::collections::HashMap;
use std::collections::HashSet;

pub fn execute(_lines: String) -> (i32, u32) {
    let coords = _lines.lines().map(|e| read_coords(&e)).collect::<Vec<(i32, i32)>>();

    let board = Board {
        x_start: coords.iter().min_by(|x, y| x.0.cmp(&y.0)).unwrap().0,
        y_start: coords.iter().min_by(|x, y| x.1.cmp(&y.1)).unwrap().1,
        x_end: coords.iter().max_by(|x, y| x.0.cmp(&y.0)).unwrap().0,
        y_end: coords.iter().max_by(|x, y| x.1.cmp(&y.1)).unwrap().1,
    };


    (largest_area(&board, &coords), region_containing_all_location(&board, &coords))
}

fn largest_area(_board: &Board, _coords: &Vec<(i32, i32)>) -> i32 {
    let mut results = HashMap::new();
    let mut border_coords = HashSet::new();
    for x in _board.x_start..=_board.x_end {
        for y in _board.y_start..=_board.y_end {
            let mut min_dist = (String::new(), _board.x_end * _board.y_end, (0, 0), 0);

            for coord in _coords {
                let dist = manhattan_distance((x, y), *coord);
                if min_dist.1 == dist {
                    min_dist = (min_dist.0, min_dist.1, min_dist.2, min_dist.3 + 1)
                } else if min_dist.1 > dist {
                    min_dist = (String::new() + &coord.0.to_string() + ";" + &coord.1.to_string(), dist, *coord, 1);
                }
            }

            if x == _board.x_start || x == _board.x_end || y == _board.y_start || y == _board.y_end {
                border_coords.insert(String::new() + &(min_dist.2).0.to_string() + ";" + &(min_dist.2).1.to_string());
            }

            if min_dist.3 == 1 && (min_dist.2).0 > _board.x_start && (min_dist.2).0 < _board.x_end
                && (min_dist.2).1 > _board.y_start && (min_dist.2).1 < _board.y_end {
                *results.entry(min_dist.0).or_insert(0) += 1
            }
        }
    }
    **results.iter()
        .filter(|e| !border_coords.contains(e.0))
        .collect::<HashMap<&String, &i32>>()
        .values()
        .max_by(|x, y| x.cmp(y)).unwrap()
}

fn region_containing_all_location(_board: &Board, _coords: &Vec<(i32, i32)>) -> u32 {
    let mut total = 0;
    for x in _board.x_start..=_board.x_end {
        for y in _board.y_start..=_board.y_end {
            let mut sum = 0;
            for coord in _coords {
                sum += manhattan_distance((x, y), *coord)
            }
            if sum < 10000 { total+=1; }
        }
    }
    total
}

fn manhattan_distance(_point1: (i32, i32), _point2: (i32, i32)) -> i32 {
    let hor = _point2.0 - _point1.0;
    let ver = _point2.1 - _point1.1;
    hor.abs() + ver.abs()
}

fn read_coords(_line: &str) -> (i32, i32) {
    let mut line = _line.replace(" ", "");
    let split = line.split(",").collect::<Vec<&str>>();
    (split.get(0).unwrap().parse().unwrap(), split.get(1).unwrap().parse().unwrap())
}


struct Board {
    x_start: i32,
    y_start: i32,
    x_end: i32,
    y_end: i32,
}
