use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;

struct Elf {
    next_elf_id: usize,
    previous_elf_id: usize,
}

fn main() {
    let input = 3014387;

    run(input, false);
    run(input, true);
}

fn run(input: usize, part_2: bool) {
    let mut elves = HashMap::new();

    (1..=input).for_each(|elf_id| {
        elves.insert(
            elf_id,
            Elf {
                next_elf_id: if elf_id == input { 1 } else { elf_id + 1 },
                previous_elf_id: if elf_id == 1 { input } else { elf_id - 1 },
            },
        );
    });

    let mut current_elf_id = 1;

    let pb = ProgressBar::new(input as u64);

    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta}) {per_sec}")
            .unwrap()
            .progress_chars("#>-"),
    );

    while elves.len() > 1 {
        let steal_from_elf_id = if !part_2 {
            elf_to_steal_from_part_1(current_elf_id, &elves)
        } else {
            elf_to_steal_from_part_2(current_elf_id, &elves)
        };
        let steal_from_elf_previous_id = elves.get(&steal_from_elf_id).unwrap().previous_elf_id;
        let steal_from_elf_next_id = elves.get(&steal_from_elf_id).unwrap().next_elf_id;

        elves
            .get_mut(&steal_from_elf_previous_id)
            .unwrap()
            .next_elf_id = steal_from_elf_next_id;

        elves
            .get_mut(&steal_from_elf_next_id)
            .unwrap()
            .previous_elf_id = steal_from_elf_previous_id;

        elves.remove(&steal_from_elf_id);

        current_elf_id = elves.get(&current_elf_id).unwrap().next_elf_id;
        pb.inc(1);
    }

    pb.finish_and_clear();

    println!("{}", current_elf_id);
}

fn elf_to_steal_from_part_1(current_elf_id: usize, elves: &HashMap<usize, Elf>) -> usize {
    elves.get(&current_elf_id).unwrap().next_elf_id
}

fn elf_to_steal_from_part_2(current_elf_id: usize, elves: &HashMap<usize, Elf>) -> usize {
    let half = elves.len() / 2;

    let mut result = current_elf_id;

    for _ in 0..half {
        result = elves.get(&result).unwrap().next_elf_id;
    }

    result
}
