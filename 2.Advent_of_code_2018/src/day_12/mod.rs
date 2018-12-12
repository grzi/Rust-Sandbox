use std::collections::HashMap;

pub fn execute(_input: String) -> (i64, i64) {
    let formatted_input = read_input(_input);
    (generation(&formatted_input,20), generation(&formatted_input,50000000000))
}

fn generation( _input : &(String, HashMap<String, String>), _iterations : u64) -> i64{
    let mut last_generation = _input.0.to_string();
    let mut last_score = 0;

    let mut deltas_repeat = HashMap::new();

    for iteration in 1..=_iterations {
        let mut iter_res = String::from("..."); // Peut s'étendre vers la gauche
        for offset in 2..last_generation.len()-2{
            let slice = &last_generation[offset - 2..=offset + 2];
            iter_res += match _input.1.get(&slice.to_string()){
                Some(val) => val,
                None => "."
            };
        }
        iter_res += "...";

        // Remarqué qu'à partir d'un moment, on est sur un pattern infini (Le schéma ne fait que se décaler
        // Si je réussi à déterminer qu'on a un delta entre le dernier score et le suivant qui se répète n fois,
        // on pourrait la multiplier avec le nombre d'itération restante ?

        let current_score = calculate_score(&iter_res,iteration);
        let current_delta = deltas_repeat.entry(current_score - last_score).or_insert(0);
        *current_delta += 1;

        if *current_delta > 50 { // Magic number power (Bon si le delta se répète 50 fois... c'est win)
            // On a trouvé le pattern ? On retourne le score actuel + les n itérations restantes * ce delta
            return current_score + (current_score - last_score) * (_iterations - iteration) as i64;
        }

        last_generation = iter_res ;
        last_score = current_score;
    };

    calculate_score(&last_generation,_iterations)

}

fn calculate_score(_current_generation : &String, _iteration : u64) -> i64{
    _current_generation
        .chars()
        .enumerate()
        .filter(|(_a, c)| c == &'#')
        .map(|(a, _b)| a as i64 - (3 + _iteration as i64))
        .sum::<i64>()
}

fn read_input(_input: String) -> (String, HashMap<String, String>) {
    let mut map = HashMap::new();

    let lines = _input.lines().collect::<Vec<&str>>();
    let pots =
        String::from("...")
            + lines.get(0).expect("at least one line is needed") + "...";
    for i in 2..lines.len() {
        let test = lines[i].split(" => ").collect::<Vec<&str>>();
        map.insert(test[0].to_string(), test[1].to_string());
    }

    (pots, map)
}
