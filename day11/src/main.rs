use crate::Atom::{DILITHEUM, ELERIUM, PLUTONIUM, PROMETHIUM, RUTHENIUM, STRONTIUM, THULIUM};
use crate::Type::{GENERATOR, MICROCHIP};
use indicatif::ProgressBar;
use std::collections::{HashSet, VecDeque};
use std::hash::Hash;

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct Configuration {
    elevator_position: u8,
    elements: Vec<(Element, u8)>,
}

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
struct Element {
    r#type: Type,
    atom: Atom,
}

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
enum Type {
    GENERATOR,
    MICROCHIP,
}

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
enum Atom {
    THULIUM,
    PLUTONIUM,
    STRONTIUM,
    PROMETHIUM,
    RUTHENIUM,
    ELERIUM,
    DILITHEUM,
}

fn main() {
    let start_config_input = Configuration {
        elevator_position: 0,
        elements: Vec::from([
            (
                Element {
                    r#type: GENERATOR,
                    atom: THULIUM,
                },
                0,
            ),
            (
                Element {
                    r#type: MICROCHIP,
                    atom: THULIUM,
                },
                0,
            ),
            (
                Element {
                    r#type: GENERATOR,
                    atom: PLUTONIUM,
                },
                0,
            ),
            (
                Element {
                    r#type: GENERATOR,
                    atom: STRONTIUM,
                },
                0,
            ),
            (
                Element {
                    r#type: MICROCHIP,
                    atom: PLUTONIUM,
                },
                1,
            ),
            (
                Element {
                    r#type: MICROCHIP,
                    atom: STRONTIUM,
                },
                1,
            ),
            (
                Element {
                    r#type: GENERATOR,
                    atom: PROMETHIUM,
                },
                2,
            ),
            (
                Element {
                    r#type: MICROCHIP,
                    atom: PROMETHIUM,
                },
                2,
            ),
            (
                Element {
                    r#type: GENERATOR,
                    atom: RUTHENIUM,
                },
                2,
            ),
            (
                Element {
                    r#type: MICROCHIP,
                    atom: RUTHENIUM,
                },
                2,
            ),
        ]),
    };

    run_simulation(start_config_input.clone());

    let mut part2 = start_config_input.clone();
    part2.elements.push((
        Element {
            r#type: GENERATOR,
            atom: ELERIUM,
        },
        0,
    ));
    part2.elements.push((
        Element {
            r#type: MICROCHIP,
            atom: ELERIUM,
        },
        0,
    ));
    part2.elements.push((
        Element {
            r#type: GENERATOR,
            atom: DILITHEUM,
        },
        0,
    ));
    part2.elements.push((
        Element {
            r#type: MICROCHIP,
            atom: DILITHEUM,
        },
        0,
    ));

    run_simulation(part2);
}

fn run_simulation(start_config: Configuration) {
    let mut checked_configs: HashSet<Configuration> = HashSet::new();
    let mut configs_to_check: VecDeque<Configuration> = VecDeque::new();
    configs_to_check.push_back(start_config);
    let mut step: u8 = 0;
    let mut left_in_current_step = 1_u64;
    let mut next_step_size = 0_u64;
    let mut pb = ProgressBar::new(left_in_current_step);

    while !configs_to_check.is_empty() {
        let config = configs_to_check.pop_front().unwrap();
        let possible_elevator_combinations = get_possible_elevator_combinations(&config);
        let mut possible_elevator_positions = Vec::new();

        if config.elevator_position < 3 {
            possible_elevator_positions.push(config.elevator_position + 1);
        }

        if config.elevator_position > 0 && elements_below_current_floor(&config) {
            possible_elevator_positions.push(config.elevator_position - 1);
        }

        for combination in possible_elevator_combinations {
            for next_elevator_position in &possible_elevator_positions {
                if is_move_allowed(&combination, *next_elevator_position, &config) {
                    let mut next_config = config.clone();
                    next_config.elevator_position = *next_elevator_position;

                    next_config
                        .elements
                        .iter_mut()
                        .filter(|(e, _)| combination.iter().any(|(c, _)| e == c))
                        .for_each(|(_, p)| *p = *next_elevator_position);

                    if !checked_configs.contains(&next_config) {
                        configs_to_check.push_back(next_config.clone());

                        next_step_size += 1;
                    }
                    checked_configs.insert(next_config);
                }
            }
        }

        if is_everything_up(&config) {
            configs_to_check.clear();
            break;
        }

        pb.inc(1);

        left_in_current_step -= 1;

        if left_in_current_step == 0 {
            pb.finish_and_clear();
            step += 1;
            left_in_current_step = next_step_size;
            next_step_size = 0;

            println!(
                "going to step {}, configs to check: {}",
                step, left_in_current_step
            );
            println!("total checked: {}", checked_configs.len());
            pb = ProgressBar::new(left_in_current_step);
        }
    }

    println!("{step}");
}

fn is_everything_up(config: &Configuration) -> bool {
    return config.elements.iter().map(|(_, v)| v).all(|v| *v == 3);
}

fn get_possible_elevator_combinations<'a>(
    config: &'a Configuration,
) -> Vec<Vec<&'a (Element, u8)>> {
    let elements_on_floor = get_elements_at_floor(config, config.elevator_position);

    let mut result = Vec::new();

    for i in 0..elements_on_floor.len() {
        result.push(vec![elements_on_floor[i]]);
        for j in i + 1..elements_on_floor.len() {
            result.push(vec![elements_on_floor[i], elements_on_floor[j]]);
        }
    }

    return result;
}

fn is_move_allowed(elements: &Vec<&(Element, u8)>, to: u8, config: &Configuration) -> bool {
    let elements_on_target_floor = get_elements_at_floor(config, to);

    return elements.iter().all(|element| {
        (element.0.r#type == GENERATOR && to > element.1) // useless to move a generator down
            || elements_on_target_floor // it's ok if there is no generator on the target floor
                .iter()
                .filter(|e| e.0.r#type == GENERATOR)
                .count()
                == 0
            || elements_on_target_floor // it's ok if there is a generator for the same element on the target floor
                .iter()
                .filter(|e| e.0.r#type == GENERATOR && e.0.atom == element.0.atom)
                .count()
                == 1
    });
}

fn get_elements_at_floor<'a>(config: &'a Configuration, floor: u8) -> Vec<&'a (Element, u8)> {
    return config
        .elements
        .iter()
        .filter(|(_, v)| *v == floor)
        .collect();
}

fn elements_below_current_floor(config: &Configuration) -> bool {
    return config
        .elements
        .iter()
        .any(|(_, v)| v < &config.elevator_position);
}
