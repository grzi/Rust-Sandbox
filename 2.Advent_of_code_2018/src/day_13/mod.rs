// J'ai inversé i et j :( roh le truc de noob... :( Mais quand même finit 600ème :P

pub fn execute(_input: String) -> ((usize, usize)) {
    let mut formatted_input = read_input(_input);
    (detect_first_crash(&mut formatted_input.0, &mut formatted_input.1))
}

fn detect_first_crash(_board: &mut Vec<Vec<Rail>>, _minecarts: &mut Vec<Minecart>) -> (usize, usize) {
    while _minecarts.iter().filter(|e| !e.boom).count() > 1 {
        _minecarts.sort_by(|x, y| {
            let val_1 = (x.coordinates.0 * 10000) + x.coordinates.1;
            let val_2 = (y.coordinates.0) * 10000 + y.coordinates.1;
            val_1.cmp(&val_2)
        });

        for i in 0.._minecarts.len() {
            if !_minecarts[i].boom {
                let next_coords = {
                    let minecart = _minecarts.get(i).expect("expect a minecart here");
                    // Assuming the direction is calculed right at last turn
                    apply_direction_to_coordinates(&minecart.coordinates, &minecart.direction)
                };

                // Is there any other minecart here ?
                if _minecarts.iter().filter(|x| !x.boom && x.coordinates == next_coords).count() > 0 {
                    _minecarts.get_mut(i).expect("expect a minecart here").boom = true;
                    println!("boom : ({}, {})", next_coords.1, next_coords.0);
                    for j in 0.._minecarts.len() {
                        if _minecarts[j].coordinates == next_coords {
                            _minecarts[j].boom = true;
                        }
                    }
                } else {
                    let mut minecart = _minecarts.get_mut(i).expect("expect a minecart here");
                    minecart.coordinates = next_coords;
                    update_minecart_directions(&mut minecart, &_board);
                }
            }
        }
    }

    let final_minecart = _minecarts.iter().filter(|e| !e.boom).next().unwrap();
    (final_minecart.coordinates.1,final_minecart.coordinates.0)
}

fn update_minecart_directions(_minecart: &mut Minecart, _board: &Vec<Vec<Rail>>) {
    match _board[_minecart.coordinates.0][_minecart.coordinates.1] {
        Rail::Turn(val) => {
            _minecart.direction = turn(val, &_minecart.direction);
        }
        Rail::Intersection => {
            _minecart.direction = {
                match _minecart.next_intersection_direction {
                    Direction::Left => {
                        let mut dir_int = _minecart.direction as i32;
                        dir_int = if dir_int == 0 {
                            3
                        } else {
                            (dir_int - 1)
                        };
                        num_to_enum(dir_int).unwrap()
                    }
                    Direction::Right => {
                        let mut dir_int = (_minecart.direction as i32 + 1) % 4;
                        num_to_enum(dir_int).unwrap()
                    }
                    _ => _minecart.direction
                }
            };

            // Update minecart next_intersection_direction
            _minecart.next_intersection_direction = match _minecart.next_intersection_direction {
                Direction::Right => Direction::Left,
                Direction::Left => Direction::Top,
                _ => Direction::Right
            };
        }
        _ => { /* Do nothing here to the dirs */ }
    }
}

fn read_input(_input: String) -> (Vec<Vec<Rail>>, Vec<Minecart>) {
    let lines = _input.lines().filter(|line| !line.is_empty()).collect::<Vec<&str>>();
    let max_line_length = lines.iter().map(|e| e.len()).max().expect("No lines, expected at least one");

    let mut board = vec![vec![Rail::Empty; max_line_length]; lines.len()];
    let mut minecarts = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        for (j, character) in line.chars().enumerate() {
            match character {
                '<' | '>' | 'v' | '^' => { // It's a minecart
                    let minecart = Minecart {
                        coordinates: (i, j),
                        direction: match character {
                            '<' => Direction::Left,
                            '>' => Direction::Right,
                            'v' => Direction::Bottom,
                            _ => Direction::Top
                        },
                        next_intersection_direction: Direction::Left,
                        boom: false
                    };
                    minecarts.push(minecart);
                    board[i][j] = Rail::Normal;
                }
                '|' | '-' => {
                    // It's a road that doesn't make the cart direction change
                    board[i][j] = Rail::Normal;
                }
                '/' | '\\' => {
                    // It's a road that will change the direction
                    board[i][j] = Rail::Turn(character);
                }
                '+' => {
                    board[i][j] = Rail::Intersection;
                }
                _ => { // It's a space
                }
            }
        }
    }

    (board, minecarts)
}

fn apply_direction_to_coordinates(_current: &(usize, usize), _dir: &Direction) -> (usize, usize) {
    match _dir {
        Direction::Top => (_current.0 - 1, _current.1),
        Direction::Right => (_current.0, _current.1 + 1),
        Direction::Bottom => (_current.0 + 1, _current.1),
        Direction::Left => (_current.0, _current.1 - 1)
    }
}

fn num_to_enum(_num: i32) -> Option<Direction> {
    match _num {
        0 => Some(Direction::Left),
        1 => Some(Direction::Top),
        2 => Some(Direction::Right),
        3 => Some(Direction::Bottom),
        _ => None,
    }
}

fn turn(_turn_val : char, _current_dir : &Direction) -> Direction {
    if _turn_val == '/' {
        match _current_dir {
            Direction::Top => Direction::Right,
            Direction::Bottom => Direction::Left,
            Direction::Left => Direction::Bottom,
            _ => Direction::Top
        }
    }else{ // '\'
        match _current_dir {
            Direction::Top => Direction::Left,
            Direction::Bottom => Direction::Right,
            Direction::Left => Direction::Top,
            _ => Direction::Bottom
        }
    }
}

#[derive(Debug)]
struct Minecart {
    direction: Direction,
    next_intersection_direction: Direction,
    coordinates: (usize, usize),
    boom : bool
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    Left = 0,
    Top = 1,
    Right = 2,
    Bottom = 3,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Rail {
    Empty,
    Normal,
    Intersection,
    Turn(char),
}
