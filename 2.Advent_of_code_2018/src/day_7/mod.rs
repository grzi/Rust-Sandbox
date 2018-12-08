use std::collections::{BTreeMap, HashMap, HashSet};
use std::borrow::BorrowMut;

pub fn execute(_lines: String) -> (String, u32) {
    let correspondances_vec = _lines.lines().map(|e| read_input(e)).collect::<Vec<(char, char)>>();
    (find_order(&correspondances_vec), find_time_with_workers(&correspondances_vec))
}

fn find_order(_correspondances_vec: &Vec<(char, char)>) -> String {
    let mut graph = get_graph(_correspondances_vec);
    let mut result = String::new();
    // graph.iter().for_each(|e| println!("{} : {:?}", e.0, e.1));
    while graph.len() != 0 {
        let mut tmp = ' ';
        for node in graph.iter() {
            if node.1.len() == 0 {
                tmp = **node.0;
                result += node.0.to_string().as_str();
                break;
            }
        }
        graph.remove(&tmp);
        graph.values_mut().for_each(|value| {
            value.remove(&tmp).to_string();
            ()
        });
    }
    result
}

fn find_time_with_workers(_correspondances_vec: &Vec<(char, char)>) -> u32 {
    let mut graph = get_graph(_correspondances_vec);
    let mut result = 0;
    let mut val = (true, 0, ' ');

    let mut workers = HashMap::new();
    for i in 0..5 {
        workers.insert(i, val);
    }

    while graph.len() != 0 {
        let mut tmp = ' ';
        for node in graph.iter() {
            if node.1.len() == 0
                && no_worker_on_this_char(&workers, **node.0)
                && worker_available(&workers) {
                add_work(&mut workers, **node.0);
            }
        }

        workers.iter().for_each(|e| print!("worker {} on {} for {}    ", e.0, (e.1).2, (e.1).1));
        println!();

        println!("graph len {}", graph.len());

        let tmp = free_next_worker(&mut workers);
        result += tmp.0;

        graph.remove(&tmp.1);
        graph.values_mut().for_each(|value| {
            value.remove(&tmp.1).to_string();
            ()
        });
    }

    result
}

fn worker_available(_workers: &HashMap<u32, (bool, u32, char)>) -> bool {
    _workers.iter().filter(|e| (e.1).0).count() > 0
}

fn add_work(_workers: &mut HashMap<u32, (bool, u32, char)>, _letter: char) {
    for mut worker in _workers.values_mut() {
        if worker.0 {
            worker.0 = false;
            worker.1 = _letter as u32 - 4;
            worker.2 = _letter;
            break;
        }
    }
}

fn no_worker_on_this_char(_workers: &HashMap<u32, (bool, u32, char)>, _letter: char) -> bool {
    _workers.values().filter(|v| v.2 == _letter).count() == 0
}

fn free_next_worker(_workers: &mut HashMap<u32, (bool, u32, char)>) -> (u32, char) {
    // trouver plus court délai / reinit ce worker / soustraire à tous les workers
    let key =
        _workers.iter().filter(|e| !(e.1).0)
            .min_by(|x, y| (x.1).1.cmp(&(y.1).1)).unwrap().0.clone();


    let mut duration = 0;
    let mut char_resetted = ' ';
    {
        let mut to_reset = _workers.entry(key).or_insert((true, 0, ' '));
        duration = to_reset.1;
        char_resetted = to_reset.2;
        to_reset.0 = true;
        to_reset.1 = 0;
        to_reset.2 = ' ';
    }


    for mut worker in _workers.values_mut() {
        if !worker.0 {
            worker.1 -= duration;
        }
    }

    (duration, char_resetted)
}

fn get_graph(_correspondances_vec: &Vec<(char, char)>) -> BTreeMap<&char, HashSet<&char>> {
    let mut graph = BTreeMap::new(); // String, (HashSet<String>)

    for correspondance in _correspondances_vec.iter() {
        graph.entry(&correspondance.1).or_insert(HashSet::new()).insert(&correspondance.0);
        graph.entry(&correspondance.0).or_insert(HashSet::new());
    }

    graph
}

fn read_input(_input: &str) -> (char, char) {
    let split = _input.split(" ").collect::<Vec<&str>>();
    (*split.get(1).unwrap().to_string().chars().collect::<Vec<char>>().get(0).unwrap(),
     *split.get(7).unwrap().to_string().chars().collect::<Vec<char>>().get(0).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(read_input(&"Step C must be finished before step A can begin."), ('C', 'A'));
    }
}
