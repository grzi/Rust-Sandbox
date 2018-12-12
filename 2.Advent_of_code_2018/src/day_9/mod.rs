use std::collections::VecDeque;

pub fn execute() -> (usize, usize) {
    (calculate_high_score(428, 72061), calculate_high_score(428, 7206100))
}

fn calculate_high_score(_player_nb: usize, _marble_number: usize) -> usize {
    let mut players_score = vec!(0; _player_nb);
    let mut ring = VecDeque::new();
    ring.push_front(0);

    for marble in 1.._marble_number {
        if marble % 23 == 0 {
            // rotate of 7 behind + delete
            (0..7).for_each(|_| {
                let tmp = ring.pop_back().expect("Rotate problem");
                ring.push_front(tmp);
            });
            players_score[marble % _player_nb] += marble + ring.pop_front().expect("No value in the ring");
        } else {
            // rotate of 2 ahead + insert
            (0..2).for_each(|_| {
                let tmp = ring.pop_front().expect("Rotate problem");
                ring.push_back(tmp);
            });
            ring.push_front(marble);
        }
    }
    *players_score.iter().max().expect("No value in the player scores")
}
