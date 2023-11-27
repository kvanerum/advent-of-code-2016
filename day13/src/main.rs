use std::collections::{HashSet, VecDeque};

fn main() {
    let input = 1364;

    move_to_target((1, 1), (31, 39), input);
}

fn move_to_target(start: (u8, u8), finish: (u8, u8), input: u32) {
    let mut visited_positions: HashSet<(u8, u8)> = HashSet::new();
    let mut positions_to_check: VecDeque<(u8, u8)> = VecDeque::new();
    positions_to_check.push_back(start);
    let mut step = 0_u8;
    let mut left_in_current_step = 1_u64;
    let mut next_step_size = 0_u64;

    while !positions_to_check.is_empty() {
        let pos = positions_to_check.pop_front().unwrap();

        if pos == finish {
            println!("{step}");
            return;
        }

        let neighbors = get_neighbors(&pos);

        for neighbor in neighbors {
            if !is_wall(neighbor, input) && !visited_positions.contains(&neighbor) {
                positions_to_check.push_back(neighbor);
                next_step_size += 1;
            }
        }

        visited_positions.insert(pos);
        left_in_current_step -= 1;
        if left_in_current_step == 0 {
            step += 1;
            left_in_current_step = next_step_size;
            next_step_size = 0;

            if step == 51 {
                println!("{}", visited_positions.len());
            }
        }
    }
}

fn get_neighbors(pos: &(u8, u8)) -> Vec<(u8, u8)> {
    let mut result = Vec::new();
    // left
    if pos.0 > 0 {
        result.push((pos.0 - 1, pos.1));
    }

    // right
    result.push((pos.0 + 1, pos.1));

    // up
    if pos.1 > 0 {
        result.push((pos.0, pos.1 - 1));
    }

    // down
    result.push((pos.0, pos.1 + 1));

    return result;
}

fn is_wall(position: (u8, u8), input: u32) -> bool {
    let x = position.0 as u32;
    let y = position.1 as u32;
    let calculation = x * x + 3 * x + 2 * x * y + y + y * y + input;
    let mut mask: u32 = 0x00000001;

    let mut bit_count = 0_u8;

    for _ in 0..32 {
        if calculation & mask > 0 {
            bit_count += 1;
        }

        mask = mask << 1;
    }

    return bit_count % 2 != 0;
}
