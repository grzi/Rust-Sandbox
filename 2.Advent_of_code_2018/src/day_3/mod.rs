use regex::Regex;
use std::collections::HashMap;

struct Claim {
    id: u32,
    start_w: u32,
    start_h: u32,
    width: u32,
    height: u32,
}

pub fn execute(_lines: String) -> (usize, u32) {
    let lines = _lines.lines().map(|e| e.trim()).collect::<Vec<&str>>();

    let mut claims = HashMap::new();

    lines.iter().for_each(
        |e| {
            let mut claim = read_input(e);
            match claim {
                Some(e) => updateClaims(&mut claims, e),
                None => println!("error at {}", e)
            }
        }
    );

    (claims.values().filter(|e| e.1 > 1).count(), get_claimer_with_overlay_nb(&mut claims))
}

fn read_input(_input: &str) -> Option<Claim> {
    let re = Regex::new(r"(#{1}[0-9]*)( @ )([0-9]*)(,)([0-9]*)(: )([0-9]*)(x)([0-9]*)").unwrap();
    if !re.is_match(_input) {
        return None;
    }

    let result = re.captures_iter(_input).next().unwrap();

    Some(
        Claim {
            id: result[1].replace("#", "").parse::<u32>().unwrap(),
            start_w: result[3].parse::<u32>().unwrap(),
            start_h: result[5].parse::<u32>().unwrap(),
            width: result[7].parse::<u32>().unwrap(),
            height: result[9].parse::<u32>().unwrap(),
        }
    )
}

fn updateClaims(_claims: &mut HashMap<String, (Vec<u32>, u32)>, _claim: Claim) {
    for i in _claim.start_w..(_claim.start_w + _claim.width) {
        for j in _claim.start_h..(_claim.start_h + _claim.height) {
            let mut current_inch = _claims.entry(String::new() + &i.to_string() + ";" + &j.to_string()).or_insert((Vec::new(), 0));
            current_inch.0.push(_claim.id);
            current_inch.1 += 1;
        }
    }
}

fn get_claimer_with_overlay_nb(_claims: &mut HashMap<String, (Vec<u32>, u32)>) -> u32 {
    let mut result = HashMap::new();

    _claims.values().for_each(
        |e| {
            if e.0.len() == 1 {
                result.entry(e.0.get(0).unwrap()).or_insert(0);
            } else {
                e.0.iter().for_each(
                    |v| {
                        *result.entry(v).or_insert(0) += 1;
                    }
                )
            }
        }
    );
    for entry in result.iter() { if *entry.1 == 0 { return **entry.0; } };
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_input_1() {
        let result = read_input("#1 @ 1,3: 4x4").unwrap();
        let expected = Claim {
            id: 1,
            start_w: 1,
            start_h: 3,
            width: 4,
            height: 4,
        };
        assert_eq!(1, result.id);
        assert_eq!(1, result.start_w);
        assert_eq!(3, result.start_h);
        assert_eq!(4, result.width);
        assert_eq!(4, result.height);
    }

    #[test]
    fn test_read_input_2() {
        let result = read_input("#123 @ 3,2: 5x4").unwrap();
        assert_eq!(123, result.id);
        assert_eq!(3, result.start_w);
        assert_eq!(2, result.start_h);
        assert_eq!(5, result.width);
        assert_eq!(4, result.height);
    }

    #[test]
    fn test_read_input_3() {
        let result = read_input("#1365 @ 824,458: 28x17").unwrap();
        assert_eq!(1365, result.id);
        assert_eq!(824, result.start_w);
        assert_eq!(458, result.start_h);
        assert_eq!(28, result.width);
        assert_eq!(17, result.height);
    }
}

// #1 @ 1,3: 4x4
