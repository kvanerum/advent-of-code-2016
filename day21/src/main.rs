use std::fs;

fn main() {
    let example = "day21/resources/example.txt";
    let input = "day21/resources/input.txt";

    let rules = read_input(input);

    println!("{}", scramble("abcdefgh", &rules));
    println!("{}", unscramble("fbgdceah", &rules));
}

fn scramble(input: &str, rules: &Vec<String>) -> String {
    let mut result = input.chars().collect::<Vec<char>>();

    for rule in rules {
        let split = rule.split_whitespace().collect::<Vec<&str>>();
        if rule.starts_with("swap position") {
            let from = split[2].parse::<usize>().unwrap();
            let to = split[5].parse::<usize>().unwrap();
            result.swap(from, to);
        } else if rule.starts_with("swap letter") {
            let letter_1 = split[2].chars().next().unwrap();
            let letter_2 = split[5].chars().next().unwrap();
            result.iter_mut().for_each(|c| {
                if *c == letter_1 {
                    *c = letter_2
                } else if *c == letter_2 {
                    *c = letter_1
                }
            });
        } else if rule.starts_with("reverse positions") {
            let from = split[2].parse::<usize>().unwrap();
            let to = split[4].parse::<usize>().unwrap();
            for i in 0..(to - from + 1) / 2 {
                result.swap(from + i, to - i);
            }
        } else if rule.starts_with("rotate left") {
            let steps = split[2].parse::<usize>().unwrap();
            result.rotate_left(steps);
        } else if rule.starts_with("rotate right") {
            let steps = split[2].parse::<usize>().unwrap();
            result.rotate_right(steps);
        } else if rule.starts_with("move position") {
            let from = split[2].parse::<usize>().unwrap();
            let to = split[5].parse::<usize>().unwrap();
            let removed = result.remove(from);
            result.insert(to, removed);
        } else if rule.starts_with("rotate based on position of letter") {
            let letter = split[6].chars().next().unwrap();
            let index = result.iter().position(|c| *c == letter).unwrap();
            let mut rotations = 1 + index;
            if index >= 4 {
                rotations += 1;
            }

            rotations %= result.len();

            result.rotate_right(rotations);
        } else {
            panic!("unknown rule: {}", rule);
        }
    }

    result.iter().collect::<String>()
}

fn unscramble(input: &str, rules: &Vec<String>) -> String {
    let mut result = input.chars().collect::<Vec<char>>();

    for rule in rules.iter().rev() {
        let split = rule.split_whitespace().collect::<Vec<&str>>();

        if rule.starts_with("swap position") {
            let from = split[2].parse::<usize>().unwrap();
            let to = split[5].parse::<usize>().unwrap();
            result.swap(from, to);
        } else if rule.starts_with("swap letter") {
            let letter_1 = split[2].chars().next().unwrap();
            let letter_2 = split[5].chars().next().unwrap();
            result.iter_mut().for_each(|c| {
                if *c == letter_1 {
                    *c = letter_2
                } else if *c == letter_2 {
                    *c = letter_1
                }
            });
        } else if rule.starts_with("reverse positions") {
            let from = split[2].parse::<usize>().unwrap();
            let to = split[4].parse::<usize>().unwrap();
            for i in 0..(to - from + 1) / 2 {
                result.swap(from + i, to - i);
            }
        } else if rule.starts_with("rotate left") {
            let steps = split[2].parse::<usize>().unwrap();
            result.rotate_right(steps);
        } else if rule.starts_with("rotate right") {
            let steps = split[2].parse::<usize>().unwrap();
            result.rotate_left(steps);
        } else if rule.starts_with("move position") {
            let to = split[2].parse::<usize>().unwrap();
            let from = split[5].parse::<usize>().unwrap();
            let removed = result.remove(from);
            result.insert(to, removed);
        } else if rule.starts_with("rotate based on position of letter") {
            let letter = split[6].chars().next().unwrap();
            let current_index = result.iter().position(|c| *c == letter).unwrap();

            let possible_previous_indices = (0..result.len())
                .filter(|&i| {
                    (i + 1 + i + (if i >= 4 { 1 } else { 0 })) % result.len() == current_index
                })
                .collect::<Vec<usize>>();

            if possible_previous_indices.len() != 1 {
                panic!(
                    "multiple possible previous indices for letter {}: {:?}",
                    letter, possible_previous_indices
                );
            }

            let previous_index = possible_previous_indices[0];

            if current_index < previous_index {
                result.rotate_right(previous_index - current_index);
            } else {
                result.rotate_left(current_index - previous_index)
            }
        } else {
            panic!("unknown rule: {}", rule);
        }
    }

    result.iter().collect::<String>()
}

fn read_input(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .expect("read input file")
        .trim_end()
        .lines()
        .map(|line| line.to_string())
        .collect()
}
