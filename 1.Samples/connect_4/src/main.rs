/*
    The connect 4 is a game played on a board with 6 lines and 7 columns
    The goal is to align diagonally, vertically or horizontally, 4 pieces.

    It's a turn by turn game. Each player will choose a column and add a piece into
    it. The piece will then fall to the bottom of this column, on top of the last piece.
*/

use std::io;

fn main() {
    connect_4();
}

fn connect_4() {
    let mut board = [[' '; 7]; 6];
    let mut player = 'X';


    display_board(&board);
    loop {
        let mut choice = String::new();

        println!("Player {}, choose your column : ", player);

        io::stdin().read_line(&mut choice)
            .expect("Impossible to read the line");

        let choice: usize = match choice.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Error : {}", e);
                continue;
            }
        };

        if add_piece(&mut board, choice, &player) {
            display_board(&board);
            match game_ended(&board, &player) {
                0 => {
                    println!("Game end : Equality");
                    break;
                },
                1 => {
                    println!("Congrats, player {} win !", &player);
                    break;
                },
                _ => {
                    change_player(&mut player);
                }
            }
        } else {
            println!("Wrong choice");
            continue;
        }
    }
}

fn display_board(_board: &[[char; 7]; 6]) {
    println!("  0  1  2  3  4  5  6");
    println!("  -  -  -  -  -  -  -");
    for lines in _board.iter() {
        print!("|");
        for column in lines.iter() {
            print!(" {} ", column);
        }
        println!("|");
    }
    println!("  -  -  -  -  -  -  -");
}

fn add_piece(_board: &mut [[char; 7]; 6], _choice: usize, _player: &char) -> bool {
    if _choice < 8 && _board[0][_choice] == ' ' {
        for i in (0..6).rev() {
            if _board[i][_choice] == ' ' {
                _board[i][_choice] = *_player;
                break;
            }
        }
        true
    } else {
        false
    }
}

fn game_ended(_board: &[[char; 7]; 6], _player: &char) -> u32 {
    if vertical_win(&_board, _player) || horizontal_win(&_board, _player)
        || diagonnal_top(&_board, _player) || diagonnal_bottom(&_board, _player) {
        return 1; // Game ended with a win
    }

    if board_is_full(_board) {
        return 0; // Board is full
    }

    return 2; // Game continue
}

fn vertical_win(_board: &[[char; 7]; 6], _player: &char) -> bool {
    for i in 0..7 {
        let mut count = 0;
        for j in 0..6 {
            count = if _board[j][i] == *_player { count + 1 } else { 0 };
            if count == 4 {
                return true;
            }
        }
    }
    false
}

fn horizontal_win(_board: &[[char; 7]; 6], _player: &char) -> bool {
    for i in 0..6 {
        let mut count = 0;
        for j in 0..7 {
            count = if _board[i][j] == *_player { count + 1 } else { 0 };
            if count == 4 {
                return true;
            }
        }
    }
    false
}

fn diagonnal_top(_board: &[[char; 7]; 6], _player: &char) -> bool {
    for i in 3..6 {
        for j in 0..4 {
            if _board[i][j] == *_player && _board[i - 1][j + 1] == *_player
                && _board[i - 2][j + 2] == *_player && _board[i - 3][j + 3] == *_player
                { return true; }
        }
    }
    false
}

fn diagonnal_bottom(_board: &[[char; 7]; 6], _player: &char) -> bool {
    for i in 3..6 {
        for j in 3..7 {
            if _board[i][j] == *_player && _board[i - 1][j - 1] == *_player
                && _board[i - 2][j - 2] == *_player && _board[i - 3][j - 3] == *_player
                { return true; }
        }
    }
    false
}

fn board_is_full(_board: &[[char; 7]; 6]) -> bool {
    let mut count = 0;
    for i in (0..7).rev() {
        if _board[0][i] != ' ' { count = count + 1; }
    }
    return count == 7;
}

fn change_player(_player: &mut char) {
    if *_player == 'X' {
        *_player = 'O'
    } else {
        *_player = 'X'
    }
}
