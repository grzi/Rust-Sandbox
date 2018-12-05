use std::collections::HashMap;
use std::collections::HashSet;

pub fn execute(_input: String)-> (u32, String){
    let lines = _input.lines().map(|e| e.trim()).collect::<Vec<&str>>();

    let mut sum = (0,0);

    for line in lines.iter() {
        let currentLineResult =
            (find_exaclty_n_repetition(line,2), find_exaclty_n_repetition(line,3));
        sum.0 += currentLineResult.0;
        sum.1 += currentLineResult.1;
    }

    (sum.0 * sum.1, find_prototyp_fabric(&lines))
}

fn find_exaclty_n_repetition(_input:&str, _n: u32 ) -> u32{
    let mut result = HashMap::new();
    _input.chars().for_each(|e| *result.entry(e).or_insert(0) += 1);

    if result.values().any(|v| *v == _n) {
         return 1;
    }
    0
}

fn find_prototyp_fabric(lines: &Vec<&str>) -> String{
    let mut oldLines = Vec::new();
    for line in lines {
        let result = get_one_digit_correspondance(&oldLines, &line);
        match result {
            Some(v) => return v,
            None =>  oldLines.push(line)
        }
    }
    String::new()
}

fn get_one_digit_correspondance(_oldLines : &Vec<&str>, _line : &str) -> Option<String> {
    let mut charsLine = _line.chars().collect::<Vec<char>>();
    for line in _oldLines{
        let mut correspondance = String::new();
        for (i,charac) in line.chars().enumerate(){
            if *charsLine.get(i).unwrap() == charac{
                correspondance += charac.to_string().as_str();
            }
        }
        if correspondance.len() == _line.len()-1{
            return Some(correspondance);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_exactly_2() {
        assert_eq!(false,find_exaclty_n_repetition("abcdef", 2));
        assert_eq!(true,find_exaclty_n_repetition("abbcdef", 2));
        assert_eq!(false,find_exaclty_n_repetition("abbbcdef", 2));
        assert_eq!(true,find_exaclty_n_repetition("abbccdef", 2));
    }

    #[test]
    fn find_exactly_3() {
        assert_eq!(false,find_exaclty_n_repetition("abcdef", 3));
        assert_eq!(false,find_exaclty_n_repetition("abbcdef", 3));
        assert_eq!(true,find_exaclty_n_repetition("abbbcdef", 3));
        assert_eq!(false,find_exaclty_n_repetition("abbccdef", 3));
    }

}

