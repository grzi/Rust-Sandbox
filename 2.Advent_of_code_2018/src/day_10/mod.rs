use regex::Regex;

pub fn execute(_lines: String) -> (usize) {
    let mut board = _lines.lines().map(|e| read_input(e)).collect::<Vec<(i32, i32, i32, i32)>>();
    for i in 0..12000{
        let max = board.iter().max_by(|x,y| x.1.cmp(&y.1)).unwrap().1;
        let min = board.iter().min_by(|x,y| x.1.cmp(&y.1)).unwrap().1;
        if max - min == 9 { //
            print_board(board.iter().map(|e| (e.0,e.1)).collect::<Vec<(i32,i32)>>());
            return i;
        }

        let tmp_board = board.iter().map(|e| (e.0 + e.2, e.1 + e.3, e.2, e.3)).collect::<Vec<(i32, i32, i32, i32)>>();
        board = tmp_board
    };
    (0)
}

fn print_board(_board:Vec<(i32,i32)>){
    let min_x = _board.iter().map(|e| e.0).min().unwrap();
    let max_x = _board.iter().map(|e| e.0).max().unwrap();
    let min_y = _board.iter().map(|e| e.1).min().unwrap();
    let max_y = _board.iter().map(|e| e.1).max().unwrap();
    for y in min_y..=max_y {
        for x in min_x..=max_x{
            if _board.iter().filter(|e| e.0 == x && e.1 == y).count() > 0{
                print!("#");
            }else{
                print!(" ");
            }
        }
        println!();
    }
}

fn read_input(_line: &str) -> (i32, i32, i32, i32) {
    let pattern: Regex = Regex::new(r"(position=<)([^,]*)(,)([^>]*)(> velocity=<)([^,]*)(,)([^>]*)(>)").unwrap();
    let groups = pattern.captures_iter(_line).next().unwrap();
    (groups[2].trim().parse().unwrap(),
     groups[4].trim().parse().unwrap(),
     groups[6].trim().parse().unwrap(),
     groups[8].trim().parse().unwrap()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_input_test() {
        assert_eq!(read_input("position=< 1,  8> velocity=< 1, -1>"), (1, 8, 1, -1));
    }
}
