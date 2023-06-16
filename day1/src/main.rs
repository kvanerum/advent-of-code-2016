use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("day1/resources/input.txt").expect("read input file");
    let movements = input.split(", ");

    part1(movements);
}

fn part1<'a>(movements: impl Iterator<Item=&'a str>) {
    let mut position = (0, 0);
    let mut current_direction: u16 = 0;
    let mut have_revisited_position = false;
    let mut visited = HashSet::new();
    visited.insert(position.clone());

    for movement in movements {
        current_direction = update_direction(current_direction, &movement[..1]);
        let size = &movement[1..].trim().parse::<i32>().expect("parse string");

        for _ in 0..*size {
            if current_direction == 0 {
                position.1 += 1;
            } else if current_direction == 90 {
                position.0 += 1;
            } else if current_direction == 180 {
                position.1 -= 1;
            } else {
                position.0 -= 1;
            }

            if !have_revisited_position && visited.contains(&position) {
                println!("first revisited distance: {}", calculate_blocks_away(&position));
                have_revisited_position = true;
            } else if !have_revisited_position {
                visited.insert(position.clone());
            }
        }
    }

    println!("{}", calculate_blocks_away(&position))
}

fn calculate_blocks_away(position: &(i32, i32)) -> i32 {
    return position.0.abs() + position.1.abs();
}

fn update_direction(current_direction: u16, movement: &str) -> u16 {
    let mut result: i16 = current_direction as i16;
    if movement == "L" {
        result -= 90;
    } else {
        result += 90;
    }

    if result < 0 {
        result = 360 + result;
    } else if result >= 360 {
        result %= 360;
    }

    return result as u16;
}
