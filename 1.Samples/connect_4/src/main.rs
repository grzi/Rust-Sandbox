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

        if add_piece(&mut board, choice - 1, &player) {
            display_board(&board);
            if game_ended(&board) {
                break;
            } else {
                change_player(&mut player);
            }
        } else {
            println!("Wrong choice");
            continue;
        }
    }
    println!("Congrats, player {} win !", &player);
}

fn display_board(_board: &[[char; 7]; 6]) {
    println!("  1  2  3  4  5  6  7");
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

//TODO : implement this
fn add_piece(_board: &mut [[char; 7]; 6], _choice: usize, _player: &char) -> bool {
    if _choice > 0 && _choice < 8 && _board[0][_choice] == ' ' {
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

//TODO : implement this
fn game_ended(_board: &[[char; 7]; 6]) -> bool {
    false
}

fn change_player(_player: &mut char) {
    if *_player == 'X' {
        *_player = 'O'
    } else {
        *_player = 'X'
    }
}
