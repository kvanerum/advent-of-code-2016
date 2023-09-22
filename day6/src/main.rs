use std::collections::HashMap;
use std::fs;

fn main() {
    let input: Vec<String> = fs::read_to_string("day6/resources/input.txt")
        .expect("read input file")
        .lines()
        .map(|s| s.to_string())
        .collect();

    let mut characters_list = Vec::new();

    for _ in 0..input.get(0).unwrap().len() {
        characters_list.push(HashMap::new());
    }

    for str in input {
        for (pos, char) in str.chars().enumerate() {
            let map: &mut HashMap<_, _> = characters_list.get_mut(pos).unwrap();

            *map.entry(char).or_insert(0) += 1;
        }
    }

    for map in &characters_list {
        let c = map
            .iter()
            .max_by(|a, b| a.1.cmp(b.1))
            .map(|item| item.0)
            .unwrap();
        print!("{}", c);
    }

    println!();

    for map in &characters_list {
        let c = map
            .iter()
            .max_by(|a, b| b.1.cmp(a.1))
            .map(|item| item.0)
            .unwrap();
        print!("{}", c);
    }
}
