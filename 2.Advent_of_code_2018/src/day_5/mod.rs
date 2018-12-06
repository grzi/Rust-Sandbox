use regex::Regex;

pub fn execute(_line: String) -> (usize, usize) {
    (reacting_polymer(&_line),shortest_polymer(&_line))
}

fn reacting_polymer(_input : &String) -> usize{
    let mut tmp = String::new();
    let mut tmp2 = _input.to_string();

    let regex = Regex::new(r"(aA|bB|cC|dD|eE|fF|gG|hH|iI|jJ|kK|lL|mM|nN|oO|pP|qQ|rR|sS|tT|uU|vV|wW|xX|yY|zZ|Aa|Bb|Cc|Dd|Ee|Ff|Gg|Hh|Ii|Jj|Kk|Ll|Mm|Nn|Oo|Pp|Qq|Rr|Ss|Tt|Uu|Vv|Ww|Xx|Yy|Zz)").unwrap();

    while tmp.len() != tmp2.len() {
        tmp = tmp2.to_string();
        tmp2 = regex.replace(tmp2.as_str(),"").to_string();
    }

    tmp.len()
}

fn shortest_polymer(_input: &String) -> usize{
    let mut res : usize = _input.len();

    for letter in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
        let tmp = _input.replace(letter, "").replace(letter.to_lowercase().next().unwrap(),"");
        let tmp_size = reacting_polymer(&tmp);
        if tmp_size < res {
            res = tmp_size
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(reacting_polymer(&"dabAcCaCBAcCcaDA".to_string()), 10);
    }

    #[test]
    fn test2() {
        assert_eq!(shortest_polymer(&"dabAcCaCBAcCcaDA".to_string()), 4);
    }
}
