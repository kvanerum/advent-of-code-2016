use std::collections::HashMap;
use std::fs;

fn main() {
    let input: Vec<String> = fs::read_to_string("day2/resources/input.txt")
        .expect("read input file")
        .lines()
        .map(|s| s.to_string())
        .collect();

    let keypad_part1: HashMap<(i8, i8), char> = HashMap::from([
        ((0, 0), '1'),
        ((1, 0), '2'),
        ((2, 0), '3'),
        ((0, 1), '4'),
        ((1, 1), '5'),
        ((2, 1), '6'),
        ((0, 2), '7'),
        ((1, 2), '8'),
        ((2, 2), '9'),
    ]);

    let keypad_part2: HashMap<(i8, i8), char> = HashMap::from([
        ((2, 0), '1'),
        ((1, 1), '2'),
        ((2, 1), '3'),
        ((3, 1), '4'),
        ((0, 2), '5'),
        ((1, 2), '6'),
        ((2, 2), '7'),
        ((3, 2), '8'),
        ((4, 2), '9'),
        ((1, 3), 'A'),
        ((2, 3), 'B'),
        ((3, 3), 'C'),
        ((2, 4), 'D'),
    ]);

    run((1, 1), keypad_part1, &input);
    println!();
    run((0, 2), keypad_part2, &input);
}

fn run(start_position: (u8, u8), keypad: HashMap<(i8, i8), char>, lines: &Vec<String>) {
    let mut position: (i8, i8) = (start_position.0 as i8, start_position.1 as i8);

    for entry in lines {
        for c in entry.chars() {
            match c {
                'U' => {
                    if keypad.contains_key(&(position.0, position.1 - 1)) {
                        position.1 -= 1;
                    }
                }
                'D' => {
                    if keypad.contains_key(&(position.0, position.1 + 1)) {
                        position.1 += 1;
                    }
                }
                'L' => {
                    if keypad.contains_key(&(position.0 - 1, position.1)) {
                        position.0 -= 1;
                    }
                }
                'R' => {
                    if keypad.contains_key(&(position.0 + 1, position.1)) {
                        position.0 += 1;
                    }
                }
                _ => {}
            }
        }

        print!("{}", keypad.get(&position).expect("exists"));
    }
}
