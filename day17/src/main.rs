use crate::Direction::{DOWN, LEFT, RIGHT, UP};
use md5;
use std::collections::{HashSet, VecDeque};

#[derive(PartialEq, Copy, Clone, Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug)]
struct State {
    position: (usize, usize),
    path: String,
}

fn main() {
    let example = "ihgpwlah";

    let password = example;
    let grid_size = 4;

    let unlocked_chars = HashSet::from(['b', 'c', 'd', 'e', 'f']);

    let mut queue = VecDeque::from([State {
        position: (0, 0),
        path: "".to_string(),
    }]);

    let mut paths = HashSet::new();

    while let Some(current_state) = queue.pop_front() {
        if current_state.position == (grid_size - 1, grid_size - 1) {
            paths.insert(current_state.path);
            continue;
        }

        let hash = format!(
            "{:x}",
            md5::compute(format!("{}{}", password, current_state.path))
        );

        get_possible_moves(current_state.position, grid_size)
            .iter()
            .filter(|direction| {
                let result = hash
                    .chars()
                    .nth(direction_to_password_position(**direction))
                    .unwrap();

                unlocked_chars.contains(&result)
            })
            .for_each(|direction| {
                let next_position = get_next_position(current_state.position, *direction);
                let next_path = format!("{}{}", current_state.path, direction_to_char(*direction));

                queue.push_back(State {
                    position: next_position,
                    path: next_path,
                })
            })
    }

    let shortest_path = paths.iter().min_by_key(|path| path.len()).unwrap();
    let longest_path = paths.iter().max_by_key(|path| path.len()).unwrap();

    println!("{}", shortest_path);
    println!("{}", longest_path.len());
}

fn get_possible_moves(current_position: (usize, usize), grid_size: usize) -> Vec<Direction> {
    let mut result = Vec::new();

    if current_position.0 > 0 {
        result.push(UP);
    }

    if current_position.0 < grid_size - 1 {
        result.push(DOWN);
    }

    if current_position.1 > 0 {
        result.push(LEFT);
    }

    if current_position.1 < grid_size - 1 {
        result.push(RIGHT);
    }

    result
}

fn get_next_position(current_position: (usize, usize), direction: Direction) -> (usize, usize) {
    match direction {
        UP => (current_position.0 - 1, current_position.1),
        DOWN => (current_position.0 + 1, current_position.1),
        LEFT => (current_position.0, current_position.1 - 1),
        RIGHT => (current_position.0, current_position.1 + 1),
    }
}

fn direction_to_char(direction: Direction) -> char {
    match direction {
        UP => 'U',
        DOWN => 'D',
        LEFT => 'L',
        RIGHT => 'R',
    }
}

fn direction_to_password_position(direction: Direction) -> usize {
    match direction {
        UP => 0,
        DOWN => 1,
        LEFT => 2,
        RIGHT => 3,
    }
}
