use regex::Regex;
use std::collections::{HashSet, VecDeque};
use std::fs;
use std::hash::Hash;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Directory {
    x: usize,
    y: usize,
    size: usize,
    used: usize,
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct State {
    empty_location: usize,
    top_right_data_location: usize,
}

fn main() {
    let input = read_input("day22/resources/input.txt");

    let mut count: usize = 0;

    for i in 0..input.len() {
        for j in 0..input.len() {
            if i != j && input[i].used > 0 && input[i].used <= (input[j].size - input[j].used) {
                count += 1;
            }
        }
    }

    println!("{}", count);

    let mut input_sorted = input.clone();
    input_sorted.sort_by(|a, b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)));

    let locked_directories: HashSet<usize> = input_sorted
        .iter()
        .enumerate()
        .filter_map(|(pos, d)| if d.used > 150 { Some(pos) } else { None })
        .collect();

    let empty_location = input_sorted.iter().position(|d| d.used == 0).unwrap();
    let width = input_sorted.iter().map(|d| d.x).max().unwrap() + 1;
    let height = input_sorted.iter().map(|d| d.y).max().unwrap() + 1;
    let initial_state = State {
        empty_location,
        top_right_data_location: width - 1,
    };
    let mut checked_states = HashSet::new();
    checked_states.insert(initial_state.clone());
    let mut next_queue = VecDeque::new();
    next_queue.push_back(initial_state);
    let mut current_steps: usize = 0;

    while !next_queue.is_empty() {
        let current_queue = next_queue;
        next_queue = VecDeque::new();

        for state in current_queue {
            if state.top_right_data_location == 0 {
                println!("found solution after {} steps", current_steps);
                return;
            }

            get_neighbours(state.empty_location, width, height)
                .iter()
                .filter(|&neighbor| !locked_directories.contains(&neighbor))
                // moving from neighbor to empty location
                .map(|&neighbor| State {
                    empty_location: neighbor,
                    top_right_data_location: if neighbor == state.top_right_data_location {
                        state.empty_location
                    } else {
                        state.top_right_data_location
                    },
                })
                .for_each(|new_state| {
                    if checked_states.insert(new_state.clone()) {
                        next_queue.push_back(new_state);
                    }
                });
        }
        current_steps += 1;
    }
}

fn get_neighbours(location: usize, width: usize, height: usize) -> Vec<usize> {
    let (x, y) = index_to_x_y(location, width);
    let mut result = Vec::new();

    // top
    if y > 0 {
        result.push(x_y_to_index(x, y - 1, width));
    }

    // right
    if x < width - 1 {
        result.push(x_y_to_index(x + 1, y, width));
    }

    // bottom
    if y < height - 1 {
        result.push(x_y_to_index(x, y + 1, width));
    }

    // left
    if x > 0 {
        result.push(x_y_to_index(x - 1, y, width));
    }

    result
}

fn index_to_x_y(index: usize, width: usize) -> (usize, usize) {
    (index % width, index / width)
}

fn x_y_to_index(x: usize, y: usize, width: usize) -> usize {
    y * width + x
}

fn read_input(filename: &str) -> Vec<Directory> {
    let re =
        Regex::new(r"^/dev/grid/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T\s+(\d+)%$").unwrap();
    fs::read_to_string(filename)
        .expect("read input file")
        .trim_end()
        .lines()
        .filter(|line| line.starts_with("/dev/grid/node-x"))
        .map(|line| {
            let caps = re.captures(line).unwrap();

            Directory {
                x: caps.get(1).unwrap().as_str().parse().unwrap(),
                y: caps.get(2).unwrap().as_str().parse().unwrap(),
                size: caps.get(3).unwrap().as_str().parse().unwrap(),
                used: caps.get(4).unwrap().as_str().parse().unwrap(),
            }
        })
        .collect()
}
