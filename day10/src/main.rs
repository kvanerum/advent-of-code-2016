use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;

struct Instruction {
    low_type: String,
    low_value: u8,
    high_type: String,
    high_value: u8,
}

fn main() {
    let (mut current_config, instructions) = parse_input();
    let mut outputs: HashMap<u8, u8> = HashMap::new();

    let mut bot_with_2_inputs = find_bot_with_2_inputs(&current_config);

    while let Some(current_bot) = bot_with_2_inputs {
        let instruction = instructions.get(&current_bot).expect("instruction");
        let current_bot_values: &mut HashSet<u8> =
            current_config.get_mut(&current_bot).expect("values");

        if current_bot_values.contains(&61) && current_bot_values.contains(&17) {
            println!("{current_bot}");
        }

        let low = *current_bot_values.iter().min().expect("value");
        let high = *current_bot_values.iter().max().expect("value");
        current_bot_values.clear();

        if instruction.low_type == "bot" {
            current_config
                .entry(instruction.low_value)
                .or_insert(HashSet::new())
                .insert(low);
        } else if instruction.low_type == "output" {
            outputs.insert(instruction.low_value, low);
        }

        if instruction.high_type == "bot" {
            current_config
                .entry(instruction.high_value)
                .or_insert(HashSet::new())
                .insert(high);
        } else if instruction.high_type == "output" {
            outputs.insert(instruction.high_value, high);
        }

        bot_with_2_inputs = find_bot_with_2_inputs(&current_config);
    }

    println!(
        "{}",
        (*outputs.get(&0).expect("value") as u32)
            * (*outputs.get(&1).expect("value") as u32)
            * (*outputs.get(&2).expect("value") as u32)
    );
}

fn find_bot_with_2_inputs(current_configuration: &HashMap<u8, HashSet<u8>>) -> Option<u8> {
    return current_configuration
        .iter()
        .find(|(_, v)| v.len() == 2)
        .map(|(k, _)| *k);
}

fn parse_input() -> (HashMap<u8, HashSet<u8>>, HashMap<u8, Instruction>) {
    let mut start_config = HashMap::new();
    let mut instructions = HashMap::new();

    let regex_value = Regex::new(r"value (\d+) goes to bot (\d+)").expect("parse regex");
    let regex_instruction =
        Regex::new(r"bot (\d+) gives low to (output|bot) (\d+) and high to (output|bot) (\d+)")
            .expect("parse regex");

    fs::read_to_string("day10/resources/input.txt")
        .expect("read input file")
        .lines()
        .for_each(|line| {
            if line.starts_with("value") {
                let capture = regex_value.captures(line).expect("parse line");
                let value = capture[1].parse::<u8>().expect("parse");
                let bot = capture[2].parse::<u8>().expect("parse");

                start_config
                    .entry(bot)
                    .or_insert(HashSet::new())
                    .insert(value);
            } else if line.starts_with("bot") {
                let capture = regex_instruction.captures(line).expect("parse line");
                let bot = capture[1].parse::<u8>().expect("parse");
                let low_type = capture[2].to_string();
                let low_value = capture[3].parse::<u8>().expect("parse");
                let high_type = capture[4].to_string();
                let high_value = capture[5].parse::<u8>().expect("parse");

                instructions.insert(
                    bot,
                    Instruction {
                        low_type,
                        low_value,
                        high_type,
                        high_value,
                    },
                );
            } else {
                panic!("invalid instruction");
            }
        });

    return (start_config, instructions);
}
